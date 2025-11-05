# toDoParcer in Rust

## Main idea
Rust parser for a custom task management with projects, dependencies, priorities, assignees, and tags. Used to describe the flow of tasks in text format.

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
