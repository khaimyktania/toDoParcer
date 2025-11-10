use anyhow::Result;
use pest::Parser;
use to_do_parcer::parser::Rule;
use to_do_parcer::{ParseError, Priority, TaskStatus, ToDoParser};

mod grammar_rule_tests {
    use super::*;

    #[test]
    fn test_parse_project_invalid_input() {
        let input = r#"
        project MyProject {
            task DoHomework;
        }
    "#;

        let pairs = ToDoParser::parse_projects(input);
        assert!(pairs.is_err(), "Parser should fail for invalid input");
    }

    #[test]
    fn test_file_rule() -> Result<()> {
        let input = r#"project "Test" { todo: "Task", }"#;
        assert!(ToDoParser::parse(Rule::file, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_project_rule() -> Result<()> {
        let input = r#"project "My Project" { }"#;
        let pair = ToDoParser::parse(Rule::project, input)?.next().unwrap();
        assert!(pair.as_str().contains("My Project"));
        Ok(())
    }

    #[test]
    fn test_task_rule() -> Result<()> {
        let input = r#"todo: "Write tests","#;
        assert!(ToDoParser::parse(Rule::task, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_todo_task_rule() -> Result<()> {
        let input = r#"todo: "Task""#;
        let pair = ToDoParser::parse(Rule::todo_task, input)?.next().unwrap();
        assert!(pair.as_str().contains("todo"));
        Ok(())
    }

    #[test]
    fn test_done_task_rule() -> Result<()> {
        let input = r#"done: "Completed""#;
        let pair = ToDoParser::parse(Rule::done_task, input)?.next().unwrap();
        assert!(pair.as_str().contains("done"));
        Ok(())
    }

    #[test]
    fn test_attribute_list_rule() -> Result<()> {
        let input = r#", @high, due: 2025-12-31"#;
        assert!(ToDoParser::parse(Rule::attribute_list, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_attribute_rule() -> Result<()> {
        assert!(ToDoParser::parse(Rule::attribute, "@high").is_ok());
        assert!(ToDoParser::parse(Rule::attribute, "due: 2025-12-31").is_ok());
        Ok(())
    }

    #[test]
    fn test_priority_rule() -> Result<()> {
        assert!(ToDoParser::parse(Rule::priority, "@high").is_ok());
        assert!(ToDoParser::parse(Rule::priority, "@medium").is_ok());
        assert!(ToDoParser::parse(Rule::priority, "@low").is_ok());
        assert!(ToDoParser::parse(Rule::priority, "@invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_due_date_rule() -> Result<()> {
        let input = "due: 2025-12-31";
        let pair = ToDoParser::parse(Rule::due_date, input)?.next().unwrap();
        assert!(pair.as_str().contains("2025-12-31"));
        Ok(())
    }

    #[test]
    fn test_assignee_rule() -> Result<()> {
        let input = "assign: @john_doe";
        let pair = ToDoParser::parse(Rule::assignee, input)?.next().unwrap();
        assert!(pair.as_str().contains("john_doe"));
        Ok(())
    }

    #[test]
    fn test_depends_on_rule() -> Result<()> {
        let input = r#"depends_on: "Setup task""#;
        let pair = ToDoParser::parse(Rule::depends_on, input)?.next().unwrap();
        assert!(pair.as_str().contains("Setup task"));
        Ok(())
    }

    #[test]
    fn test_tag_rule() -> Result<()> {
        let input = r#"@tag: "bug""#;
        let pair = ToDoParser::parse(Rule::tag, input)?.next().unwrap();
        assert!(pair.as_str().contains("bug"));
        Ok(())
    }

    #[test]
    fn test_quoted_rule() -> Result<()> {
        assert_eq!(
            ToDoParser::parse(Rule::quoted, r#""Test String""#)?
                .next()
                .unwrap()
                .as_str(),
            r#""Test String""#
        );
        assert_eq!(
            ToDoParser::parse(Rule::quoted, r#""""#)?
                .next()
                .unwrap()
                .as_str(),
            r#""""#
        );
        assert!(ToDoParser::parse(Rule::quoted, r#""unclosed"#).is_err());
        Ok(())
    }

    #[test]
    fn test_identifier_rule() -> Result<()> {
        assert_eq!(
            ToDoParser::parse(Rule::identifier, "user_name")?
                .next()
                .unwrap()
                .as_str(),
            "user_name"
        );
        assert_eq!(
            ToDoParser::parse(Rule::identifier, "user-123")?
                .next()
                .unwrap()
                .as_str(),
            "user-123"
        );
        assert_eq!(
            ToDoParser::parse(Rule::identifier, "abc123")?
                .next()
                .unwrap()
                .as_str(),
            "abc123"
        );
        Ok(())
    }

    #[test]
    fn test_date_rule() -> Result<()> {
        assert_eq!(
            ToDoParser::parse(Rule::date, "2025-12-31")?
                .next()
                .unwrap()
                .as_str(),
            "2025-12-31"
        );
        assert!(ToDoParser::parse(Rule::date, "2025/12/31").is_err());
        assert!(ToDoParser::parse(Rule::date, "25-12-31").is_err());
        Ok(())
    }

    #[test]
    fn test_comment_rule() -> Result<()> {
        let input = r#"
        // This is a comment
        project "Test" {
            todo: "Task",
        }
        "#;
        assert!(ToDoParser::parse(Rule::file, input).is_ok());
        Ok(())
    }
}

mod integration_tests {
    use super::*;

    #[test]
    fn empty_project() {
        let p = ToDoParser::parse_projects(r#"project "Empty" {}"#).unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].name, "Empty");
        assert_eq!(p[0].tasks.len(), 0);
    }

    #[test]
    fn single_todo_task() {
        let p = ToDoParser::parse_projects(r#"project "Test" { todo: "Task", }"#).unwrap();
        assert_eq!(p[0].tasks[0].title, "Task");
        assert_eq!(p[0].tasks[0].status, TaskStatus::Todo);
    }

    #[test]
    fn single_done_task() {
        let p = ToDoParser::parse_projects(r#"project "Test" { done: "Done", }"#).unwrap();
        assert_eq!(p[0].tasks[0].status, TaskStatus::Done);
    }

    #[test]
    fn priorities() {
        let p = ToDoParser::parse_projects(
            r#"project "Test" {
            todo: "H", @high,
            todo: "M", @medium,
            todo: "L", @low,
        }"#,
        )
        .unwrap();
        assert_eq!(p[0].tasks[0].priority, Some(Priority::High));
        assert_eq!(p[0].tasks[1].priority, Some(Priority::Medium));
        assert_eq!(p[0].tasks[2].priority, Some(Priority::Low));
    }

    #[test]
    fn due_dates() {
        let p = ToDoParser::parse_projects(
            r#"project "Test" {
            todo: "T1", due: 2025-01-15,
            todo: "T2", due: 2025-06-30,
        }"#,
        )
        .unwrap();
        assert_eq!(p[0].tasks[0].due_date, Some("2025-01-15".to_string()));
        assert_eq!(p[0].tasks[1].due_date, Some("2025-06-30".to_string()));
    }

    #[test]
    fn assignees() {
        let p = ToDoParser::parse_projects(
            r#"project "Test" {
            todo: "T", assign: @john_doe,
        }"#,
        )
        .unwrap();
        assert_eq!(p[0].tasks[0].assignee, Some("john_doe".to_string()));
    }

    #[test]
    fn dependencies() {
        let p = ToDoParser::parse_projects(
            r#"project "Test" {
            todo: "A",
            todo: "B", depends_on: "A",
        }"#,
        )
        .unwrap();
        assert_eq!(p[0].tasks[1].depends_on, Some("A".to_string()));
    }

    #[test]
    fn tags() {
        let p = ToDoParser::parse_projects(
            r#"project "Test" {
            todo: "T", @tag: "bug", @tag: "urgent",
        }"#,
        )
        .unwrap();
        assert_eq!(p[0].tasks[0].tags.len(), 2);
        assert!(p[0].tasks[0].tags.contains(&"bug".to_string()));
        assert!(p[0].tasks[0].tags.contains(&"urgent".to_string()));
    }

    #[test]
    fn all_attributes_combined() {
        let p = ToDoParser::parse_projects(r#"project "Test" {
            todo: "Complex", @high, due: 2025-12-31, assign: @alice, depends_on: "Prev", @tag: "important",
        }"#).unwrap();
        let t = &p[0].tasks[0];
        assert_eq!(t.title, "Complex");
        assert_eq!(t.status, TaskStatus::Todo);
        assert_eq!(t.priority, Some(Priority::High));
        assert_eq!(t.due_date, Some("2025-12-31".to_string()));
        assert_eq!(t.assignee, Some("alice".to_string()));
        assert_eq!(t.depends_on, Some("Prev".to_string()));
        assert_eq!(t.tags.len(), 1);
    }

    #[test]
    fn multiple_tasks() {
        let p = ToDoParser::parse_projects(
            r#"project "Test" {
            todo: "T1",
            done: "T2",
            todo: "T3", @high,
        }"#,
        )
        .unwrap();
        assert_eq!(p[0].tasks.len(), 3);
        assert_eq!(p[0].tasks[0].status, TaskStatus::Todo);
        assert_eq!(p[0].tasks[1].status, TaskStatus::Done);
        assert_eq!(p[0].tasks[2].priority, Some(Priority::High));
    }

    #[test]
    fn multiple_projects() {
        let p = ToDoParser::parse_projects(
            r#"
        project "P1" { todo: "A", }
        project "P2" { todo: "B", done: "C", }
        "#,
        )
        .unwrap();
        assert_eq!(p.len(), 2);
        assert_eq!(p[0].name, "P1");
        assert_eq!(p[1].name, "P2");
        assert_eq!(p[0].tasks.len(), 1);
        assert_eq!(p[1].tasks.len(), 2);
    }

    #[test]
    fn unicode_support() {
        let p = ToDoParser::parse_projects(
            r#"project "Проект" {
            todo: "Завдання 🚀",
        }"#,
        )
        .unwrap();
        assert_eq!(p[0].name, "Проект");
        assert_eq!(p[0].tasks[0].title, "Завдання 🚀");
    }

    #[test]
    fn special_characters() {
        let p = ToDoParser::parse_projects(
            r#"project "Test" {
            todo: "Chars: !@#$%^&*()",
        }"#,
        )
        .unwrap();
        assert_eq!(p[0].tasks[0].title, "Chars: !@#$%^&*()");
    }

    #[test]
    fn minimal_whitespace() {
        let p = ToDoParser::parse_projects(r#"project "T"{todo:"X",}"#).unwrap();
        assert_eq!(p[0].tasks.len(), 1);
    }

    #[test]
    fn excessive_whitespace() {
        let p = ToDoParser::parse_projects(
            r#"
        project   "T"   {
            todo:    "X"   ,   @high   ,
        }
        "#,
        )
        .unwrap();
        assert_eq!(p[0].tasks[0].priority, Some(Priority::High));
    }

    #[test]
    fn with_comments() {
        let p = ToDoParser::parse_projects(
            r#"
        // Comment
        project "Test" {
            // Another comment
            todo: "Task", // Inline
        }
        "#,
        )
        .unwrap();
        assert_eq!(p[0].tasks.len(), 1);
    }

    #[test]
    fn attribute_order_independence() {
        let p1 = ToDoParser::parse_projects(
            r#"project "T" { todo: "X", @high, due: 2025-12-31, assign: @u, }"#,
        )
        .unwrap();
        let p2 = ToDoParser::parse_projects(
            r#"project "T" { todo: "X", assign: @u, @high, due: 2025-12-31, }"#,
        )
        .unwrap();
        assert_eq!(p1[0].tasks[0].priority, p2[0].tasks[0].priority);
        assert_eq!(p1[0].tasks[0].due_date, p2[0].tasks[0].due_date);
        assert_eq!(p1[0].tasks[0].assignee, p2[0].tasks[0].assignee);
    }

    #[test]
    fn realistic_workflow() {
        let p = ToDoParser::parse_projects(
            r#"
        project "Sprint" {
            todo: "Design", @high, due: 2025-11-15, assign: @designer, @tag: "frontend",
            todo: "Auth", @high, due: 2025-11-20, depends_on: "DB", @tag: "backend",
            done: "DB", @medium, assign: @dev,
        }
        "#,
        )
        .unwrap();
        assert_eq!(p[0].tasks.len(), 3);
        let completed = p[0]
            .tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Done)
            .count();
        assert_eq!(completed, 1);
    }
}

mod error_tests {
    use super::*;

    #[test]
    fn missing_comma() {
        assert!(
            ToDoParser::parse_projects(
                r#"project "T" {
            todo: "T1"
            todo: "T2",
        }"#
            )
            .is_err()
        );
    }

    #[test]
    fn invalid_date_format() {
        assert!(
            ToDoParser::parse_projects(
                r#"project "T" {
            todo: "X", due: 2025/12/31,
        }"#
            )
            .is_err()
        );
    }

    #[test]
    fn unclosed_quote() {
        assert!(
            ToDoParser::parse_projects(
                r#"project "Test {
            todo: "Task",
        }"#
            )
            .is_err()
        );
    }

    #[test]
    fn missing_braces() {
        assert!(
            ToDoParser::parse_projects(
                r#"project "Test"
            todo: "Task",
        "#
            )
            .is_err()
        );
    }

    #[test]
    fn empty_input() {
        assert!(ToDoParser::parse_projects("").is_err());
    }

    #[test]
    fn whitespace_only() {
        assert!(ToDoParser::parse_projects("   \n\t  \n  ").is_err());
    }

    #[test]
    fn nonexistent_file() {
        let result = ToDoParser::parse_from_file("nonexistent.txt");
        assert!(result.is_err());
        match result {
            Err(ParseError::Io(_)) => {}
            _ => panic!("Expected IO error"),
        }
    }
}
