use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct ToDoParser;

impl ToDoParser {
    pub fn parse_project(
        input: &str,
    ) -> Result<pest::iterators::Pairs<'_, Rule>, pest::error::Error<Rule>> {
        Self::parse(Rule::project, input)
    }
}

// fn main() {
//     println!("ToDoFlow parser works!");
// }
