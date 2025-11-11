# to_do_parcer

A Rust parser and CLI for a custom text-based task management format.  
It supports **projects**, **tasks**, **dependencies**, **priorities**, **assignees**, and **tags** —  
and allows parsing structured `.todo` files into a clear, typed Abstract Syntax Tree (AST).

---

## Overview

The parser processes plain-text files describing projects and their tasks.  
Each project can include tasks marked as `todo:` or `done:`, with optional metadata attributes.

It can:
- Parse and display structured project data in console output
- Handle attributes like `@high`, `due:YYYY-MM-DD`, `assign:@user`
- Detect syntax errors and invalid formatting
- Be used as both a **CLI tool** and a **Rust library**

---

## Example Input

```text
project "Parser" {
  todo: "Design grammar", @high, due:2025-11-15, assign:@tanya, @tag:"core"
  todo: "Write parser in Rust", depends_on:"Design grammar", assign:@oleksii
  done: "Initialize Cargo project", @low, @tag:"setup"
} ```

```
Project: Parser

[TODO] Design grammar
       Priority: High
       Due: 2025-11-15
       Assigned to: @tanya
       Tag: core

[TODO] Write parser in Rust
       Depends on: Design grammar
       Assigned to: @oleksii

[DONE] Initialize Cargo project
       Priority: Low
       Tag: setup

-----------------------------------
Total: 3 tasks (2 active, 1 completed)
```
---

## CLI Usage

--- 
You can run the parser as a command-line tool.

```
# Parse a file and print results
to_do_parcer parse --file examples/project.txt

# Show parse tree for debugging
to_do_parcer parse --file examples/project.txt --tree

# Show author info
to_do_parcer credits
```

## Library Example

The parser can also be used as a library in your Rust code.

```
use to_do_parcer::ToDoParser;

let input = r#"
project "Parser" {
  todo: "Design grammar", @high, due:2025-11-15, assign:@tanya
}
"#;

let projects = ToDoParser::parse_projects(input).unwrap();
println!("{:#?}", projects);
``` 

## Errors

Common parsing errors and their causes:
--- 

Error
```
Parsing failed: expected project	
```
Input doesn’t match grammar

```
File reading error: ...	
File not found or unreadable
````
Invalid date format	Wrong format (must be YYYY-MM-DD)