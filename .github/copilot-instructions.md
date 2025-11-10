## Repo snapshot and purpose

This crate (`to_do_parcer`) is a small Rust CLI tool that parses a custom text format describing projects and tasks and prints a formatted view (or a pest syntax tree).

Key files:
- `toDoParcer/toDoParcer/src/main.rs` — CLI (clap) and entrypoint. Subcommands: `Parse` (file + --tree) and `Credits`.
- `toDoParcer/toDoParcer/src/parser.rs` — pest parser implementation and public API: `ToDoParser`, `ParseError`, `Project`, `Task`, `Priority`, `TaskStatus`. Use `ToDoParser::parse_projects()` to get `Vec<Project>`.
- `toDoParcer/toDoParcer/src/lib.rs` — re-exports and public surface used by `main.rs`.
- `toDoParcer/toDoParcer/src/grammar.pest` — grammar definition. Modifying it changes the parsing behavior.
- `toDoParcer/examples/project.txt` — example input files useful for tests and manual runs.
- `toDoParcer/tests/` — contains unit tests (run with `cargo test`).

## Big-picture architecture

- Single crate binary and library combined. `main.rs` is small and delegates parsing and display to `parser.rs` / library types.
- Parsing: `pest` (grammar in `grammar.pest`) -> `ToDoParser` (derived with `#[derive(pest_derive::Parser)]`) -> helpers in `parser.rs` convert pest pairs into `Project` and `Task` structs.
- Presentation: `Project::display()` handles console formatting. Tests and examples rely on this formatting.

## How to run and debug locally (Windows / PowerShell)

1. Change to the crate directory:

```powershell
cd .\toDoParcer\toDoParcer
```

2. Build:

```powershell
cargo build
```

3. Run the example parser (text input in `examples/project.txt`):

```powershell
cargo run -- parse --file .\examples\project.txt
```

4. Print the pest syntax tree (useful when changing grammar):

```powershell
cargo run -- parse --file .\examples\project.txt --tree
```

5. Tests:

```powershell
cargo test
```

## Project-specific conventions and patterns agents should follow

- Do not change `main.rs` CLI flags names or semantics without updating README and tests. The CLI expects `Parse` + `--file` and optional `--tree`.
- The parser API surface is in `parser.rs` and re-exported in `lib.rs`. Use `ToDoParser::parse_projects(&str)` for unit tests and tools that need the in-memory model.
- Formatting/printing is centralized in `Project::display()` — prefer adjusting display logic there instead of changing callers.
- When modifying grammar (`grammar.pest`):
  - Update parsing helper functions in `parser.rs` (e.g., `parse_task`, `parse_attribute`) to match rule changes.
  - Add or update unit tests under `tests/` and add an example in `examples/project.txt` demonstrating the new/changed syntax.
  - Use `cargo run -- parse --file ... --tree` to visually inspect pest pair nesting — the repo already contains a `display_tree` helper (enabled with debug_assertions).

## Integration points and error handling

- IO and parsing errors are wrapped in `ParseError` (enum with conversions for IO and pest errors). Callers (e.g., `main.rs`) expect `Result<_, ParseError>`.
- Public model types: `Project`, `Task`, `Priority`, `TaskStatus` are used across the crate. Keep their public fields stable or update all usages.

## Small examples to reference in edits

- Programmatic parse and display:

```rust
let content = std::fs::read_to_string("examples/project.txt")?;
let projects = ToDoParser::parse_projects(&content)?;
for p in projects { p.display(); }
```

- Get a pest parse tree for debugging:

```rust
let pairs = ToDoParser::parse(Rule::file, &content)?;
display_tree(pairs);
```

## When adding features

- Add unit tests to `tests/` demonstrating new syntax and expected printed output.
- Keep grammar changes minimal and well-documented in a short comment at top of `grammar.pest`.
- If you change public types or error variants, update `README.md` and tests accordingly.

## Where to look when things break

- `grammar.pest` and `parser.rs` — most parsing bugs.
- `Project::display()` in `parser.rs` — formatting/console issues.
- `main.rs` — CLI wiring and error reporting.

---

If any area above is unclear or you'd like more detail (e.g., recommended unit test assertions, example inputs to exercise edge cases, or a short dev workflow script), tell me which part and I'll expand or iterate.
