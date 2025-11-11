# toDoParcer in Rust

## Main idea
Rust parser for a custom task management with projects, dependencies, priorities, assignees, and tags. Used to describe the flow of tasks in text format.

- docs: https://docs.rs/to_do_parcer/latest/to_do_parcer/ 
- crate: https://crates.io/crates/to_do_parcer

It includes:
- projects with tasks and their statuses (todo, done);
- set priority, deadline, executor;
- add tags and dependencies between tasks;
- convert text to JSON, visualize the flow graph or work with it via CLI.

## Technical description of the parsing process

The program reads text files with a project description, which may contain:

- project name (project "...");
- tasks of the todo: or done: type;
- additional attributes:
- priority (@high, @medium, @low);
- due date (due:2025-11-10);
- executor (assign:@user);
- dependencies between tasks (depends_on:"...");
- tags (@tag:).

## How parsing works

The pest library is used, which, based on the previously described grammar, determines the main thing in the document. The program forms a tree of elements in which each node corresponds to a separate object - a project, task, or attribute. After analysis, the user sees the result in console format.

Example:

file ->
```
project "Parser" { 
  todo: "Design grammar" @high due:2025-11-15 assign:@tanya 
  todo: "Write parser in Rust" depends_on:"Design grammar" 
  assign:@oleksii done: "Initialize Cargo project" }
```

Project: Parser
-----------------------------------     
```
[TODO] Design grammar Priority: 
       High Due: 2025-11-15
       Assigned to: @tanya 
       
[TODO] Write parser in Rust 
       Depends on: Design grammar 
       Assigned to: @oleksii 
[DONE] Initialize Cargo project

----------------------------------- 
Total: 3 tasks (2 active, 1 completed)
```

If there are syntax errors in the text (for example, a missing parenthesis or an incorrect date), the parser will report this to the console.
Output to the console in a clear tabular form (as in the example above).


## Grammar 

```
/// The root rule — represents the entire file.
/// 
/// Each file must contain one or more `project` blocks.
file = { SOI ~ project+ ~ EOI }

/// Defines a block (project) of tasks
project = { 
    "project" ~ quoted 
    ~ "{" 
    ~ task*
    ~ "}" 
}

/// Kind of task
task = { (todo_task | done_task) ~ "," }

/// A task that is still pending.
todo_task = { "todo:" ~ quoted ~ attribute_list }

/// A task that has been completed.
done_task = { "done:" ~ quoted ~ attribute_list }

/// Optional list of attributes associated with a task.
attribute_list = { ("," ~ attribute)* }

/// Possible attributes for a task: priority, due date, assignee, dependencies, tags.
attribute = { priority | due_date | assignee | depends_on | tag }

/// Priority marker for a task.
priority = { "@high" | "@medium" | "@low" }

/// Task due date in YYYY-MM-DD format.
due_date = { "due:" ~ date }
assignee = { "assign:" ~ "@" ~ identifier }
depends_on = { "depends_on:" ~ quoted }
tag = { "@tag:" ~ quoted }

// Quoted string: "Something"
quoted = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
identifier = @{ (ASCII_ALPHANUMERIC | "_" | "-")+ }
date = @{ ASCII_DIGIT{4} ~ "-" ~ ASCII_DIGIT{2} ~ "-" ~ ASCII_DIGIT{2} }

// Whitespace (space or tab)
WHITESPACE = _{ " " | "\t" | "\r\n" | "\n" }

// Line breaks
NEWLINE = _{ "\r"? ~ "\n" }
COMMENT = _{ "//" ~ (!NEWLINE ~ ANY)* ~ NEWLINE? }
```
