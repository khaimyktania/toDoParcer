use toDoParcer::parser::ToDoParser;
use pest::Parser;

// #[test]
// fn test_parse_project_valid_input() {
//     let input = r#"project "P" { }"#;

//     let pairs = ToDoParser::parse_project(input);
//     assert!(pairs.is_ok(), "Parser should succeed for valid input, got {:?}", pairs);
// }

#[test]
fn test_parse_project_invalid_input() {
    let input = r#"
        project MyProject {
            task DoHomework;
        }
    "#;

    let pairs = ToDoParser::parse_project(input);
    assert!(pairs.is_err(), "Parser should fail for invalid input");
}
