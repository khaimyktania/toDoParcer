use pest::iterators::{Pair, Pairs};
use pest::Parser;
use thiserror::Error;

/// A parser implementation for the custom file format using Pest.
///
/// This struct implements the `pest::Parser` trait .
#[derive(pest_derive::Parser)]
// #[doc = include_str!("grammar.pest")]
#[grammar = "grammar.pest"]
pub struct ToDoParser;

/// Errors that can occur while reading input or parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Error returned when file cannot be read.
    #[error("File reading error: {0}")]
    Io(#[from] std::io::Error),

    /// Error returned when Pest parser fails.
    #[error("Parsing failed: {0}")]
    Pest(#[from] Box<pest::error::Error<Rule>>),
}

/// A project node in the AST containing the main things: a
/// name and a list of tasks.
#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub tasks: Vec<Task>,
}

/// A task node in the AST representing an individual task with its attributes.
#[derive(Debug, Clone)]
pub struct Task {
    pub status: TaskStatus,
    pub title: String,
    pub priority: Option<Priority>,
    pub due_date: Option<String>,
    pub assignee: Option<String>,
    pub depends_on: Option<String>,
    pub tags: Vec<String>,
}

/// The status of a task, either Todo or Done.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    Done,
}

/// The priority level of a task.
#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Project {
    /// Display the project and its tasks in a normal format.
    ///
    /// # Example
    /// ``````
    /// project.display();
    /// ```
    pub fn display(&self) {
        println!("Project: {}\n", self.name);

        for task in &self.tasks {
            let status = match task.status {
                TaskStatus::Todo => "[TODO]",
                TaskStatus::Done => "[DONE]",
            };
            println!("{} {}", status, task.title);

            if let Some(priority) = &task.priority {
                let p = match priority {
                    Priority::High => "High",
                    Priority::Medium => "Medium",
                    Priority::Low => "Low",
                };
                println!("       Priority: {}", p);
            }

            if let Some(due) = &task.due_date {
                println!("       Due: {}", due);
            }

            if let Some(assignee) = &task.assignee {
                println!("       Assigned to: @{}", assignee);
            }

            if let Some(depends) = &task.depends_on {
                println!("       Depends on: {}", depends);
            }

            for tag in &task.tags {
                println!("       Tag: {}", tag);
            }

            println!();
        }

        let total = self.tasks.len();
        let completed = self
            .tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Done)
            .count();
        let active = total - completed;

        println!("-----------------------------------");
        println!(
            "Total: {} tasks ({} active, {} completed)",
            total, active, completed
        );
    }
}

/// Methods for the ToDoParser to parse projects and tasks from input strings or files.
impl ToDoParser {
    /// Parse projects from a given input string.
    ///
    /// # Arguments
    /// * `input` - Text representation of the projects and tasks
    ///
    /// # Returns
    /// * `Ok(Vec<Project>)` on success
    /// * `Err(ParseError)` if parsing fails
    ///
    /// # Example
    /// ```
    /// let projects = ToDoParser::parse_projects(input)?;
    /// ```
    pub fn parse_projects(input: &str) -> Result<Vec<Project>, ParseError> {
        let pairs = Self::parse(Rule::file, input).map_err(|e| ParseError::Pest(Box::new(e)))?;
        let mut projects = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::file => {
                    for inner in pair.into_inner() {
                        if inner.as_rule() == Rule::project {
                            projects.push(parse_project_pair(inner));
                        }
                    }
                }
                Rule::project => {
                    projects.push(parse_project_pair(pair));
                }
                _ => {}
            }
        }

        Ok(projects)
    }

    /// Parse projects from a file into structured data.
    ///
    /// # Arguments
    /// * `path` - Path to the input file
    ///
    /// # Returns
    /// * `Ok(Vec<Project>)` on success
    /// * `Err(ParseError)` if reading or parsing fails
    ///
    /// # Example
    /// ```
    /// let projects = ToDoParser::parse_from_file("tasks.txt")?;
    /// ```
    pub fn parse_from_file(path: &str) -> Result<Vec<Project>, ParseError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_projects(&content)
    }
}

/// Converts a Pest `project` pair into a `Project` struct.
///
/// # Arguments
/// * `pair` - A Pest `Pair` representing a project
///
/// # Returns
/// * `Project` -- struct with parsed data
fn parse_project_pair(pair: Pair<Rule>) -> Project {
    let mut project_name = String::new();
    let mut tasks = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::quoted => project_name = parse_quoted(inner),
            Rule::task => tasks.push(parse_task(inner)),
            _ => {}
        }
    }

    Project {
        name: project_name,
        tasks,
    }
}

/// Extracts string content without quotes
///
/// /// # Arguments
/// * `pair` - Pest `Pair` representing a quoted string
///
/// # Returns
/// * `String` representing the raw content
fn parse_quoted(pair: Pair<Rule>) -> String {
    pair.as_str().trim_matches('"').to_string()
}

/// Converts a Pest `task` pair into a `Task` struct.
///
/// # Arguments
/// * `pair` - A Pest `Pair` representing a task
///
/// # Returns
/// * `Task` -- struct with parsed data
fn parse_task(pair: Pair<Rule>) -> Task {
    let mut task = Task {
        status: TaskStatus::Todo,
        title: String::new(),
        priority: None,
        due_date: None,
        assignee: None,
        depends_on: None,
        tags: Vec::new(),
    };

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::todo_task | Rule::done_task => {
                task.status = if inner.as_rule() == Rule::done_task {
                    TaskStatus::Done
                } else {
                    TaskStatus::Todo
                };
                parse_task_details(
                    inner,
                    &mut task.title,
                    &mut task.priority,
                    &mut task.due_date,
                    &mut task.assignee,
                    &mut task.depends_on,
                    &mut task.tags,
                );
            }
            _ => {}
        }
    }

    task
}

/// Parses details and attributes of a single task.`.
///
/// # Arguments
/// * `pair` — Pest pair for the task block.
/// * `title`, `priority`, `due_date`, `assignee`, `depends_on`, `tags` — Mutable references to fill parsed data.

fn parse_task_details(
    pair: Pair<Rule>,
    title: &mut String,
    priority: &mut Option<Priority>,
    due_date: &mut Option<String>,
    assignee: &mut Option<String>,
    depends_on: &mut Option<String>,
    tags: &mut Vec<String>,
) {
    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::quoted => *title = parse_quoted(item),
            Rule::attribute_list => {
                for attr in item.into_inner().filter(|a| a.as_rule() == Rule::attribute) {
                    parse_attribute(attr, priority, due_date, assignee, depends_on, tags);
                }
            }
            _ => {}
        }
    }
}

/// Parses a single attribute of a task (priority, due date, etc.).
///
/// # Arguments
/// * `pair` — Pest pair for the attribute.
/// * `priority`, `due_date`, `assignee`, `depends_on`, `tags` — Mutable references to fill parsed data.
fn parse_attribute(
    pair: Pair<Rule>,
    priority: &mut Option<Priority>,
    due_date: &mut Option<String>,
    assignee: &mut Option<String>,
    depends_on: &mut Option<String>,
    tags: &mut Vec<String>,
) {
    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::priority => {
                *priority = match item.as_str() {
                    "@high" => Some(Priority::High),
                    "@medium" => Some(Priority::Medium),
                    "@low" => Some(Priority::Low),
                    _ => None,
                };
            }
            Rule::due_date => {
                if let Some(date) = item.into_inner().find(|i| i.as_rule() == Rule::date) {
                    *due_date = Some(date.as_str().to_string());
                }
            }
            Rule::assignee => {
                if let Some(id) = item.into_inner().find(|i| i.as_rule() == Rule::identifier) {
                    *assignee = Some(id.as_str().to_string());
                }
            }
            Rule::depends_on => {
                if let Some(dep) = item.into_inner().find(|i| i.as_rule() == Rule::quoted) {
                    *depends_on = Some(parse_quoted(dep));
                }
            }
            Rule::tag => {
                for tag_item in item.into_inner().filter(|i| i.as_rule() == Rule::quoted) {
                    tags.push(parse_quoted(tag_item));
                }
            }
            _ => {}
        }
    }
}

/// Debug utility: prints a tree of parsed rules (only in debug mode).
///
/// # Arguments
/// * `pairs` - Pest parse tree to display.   
#[cfg(debug_assertions)]
pub fn display_tree(pairs: Pairs<Rule>) {
    fn print_pair(pair: Pair<Rule>, indent: usize) {
        println!("{:indent$}- {:?}", "", pair.as_rule(), indent = indent * 2);
        for inner in pair.into_inner() {
            print_pair(inner, indent + 1);
        }
    }

    for pair in pairs {
        print_pair(pair, 0);
    }
}
