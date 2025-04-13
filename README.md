# Strux

`strux` is a command-line utility written in Rust that parses specially formatted markdown files to generate directory structures and file contents. It's useful for:

* **Project Bootstrapping:** Quickly create initial project layouts from a template.
* **Documentation:** Document file structures and provide their content within a single markdown file.
* **Reproducible Setups:** Define file layouts declaratively.

The tool reads a markdown file, identifies actions (like creating or deleting files) based on specific header formats, and executes these actions relative to a specified output directory.

## Features

* Supports creating files with content defined in code blocks.
* Supports deleting files.
* Multiple header formats for defining actions (Markdown headers, backticks, internal comments).
* "Wrapped" header format for associating headers with subsequent code blocks.
* Automatic creation of parent directories.
* Safety checks to prevent writing outside the target base directory.
* Option to force overwriting existing files.
* Detailed summary output of actions performed, skipped, or failed.
* Pre-commit hooks configured for code quality and consistency.

## Installation

### From Source

1. Ensure you have Rust and Cargo installed ([https://rustup.rs/](https://rustup.rs/)).
2. Clone the repository:

    ```bash
    git clone https://github.com/romelium/strux.git
    cd strux
    ```

3. Build and install the binary:

    ```bash
    cargo install --path .
    ```

    This will install the `strux` executable into your Cargo bin directory (usually `~/.cargo/bin/`).

### From Crates.io (Once Published)

```bash
cargo install strux
```

### From GitHub Releases

Pre-compiled binaries for Linux (x86_64), macOS (x86_64, arm64), and Windows (x86_64) are available on the [GitHub Releases page](https://github.com/romelium/strux/releases). Download the appropriate archive for your system, extract it, and place the `strux` (or `strux.exe`) executable in a directory included in your system's `PATH`.

## Usage

```bash
strux [OPTIONS] <MARKDOWN_FILE>
```

**Arguments:**

* `<MARKDOWN_FILE>`: Path to the input markdown file containing the file structure definitions.

**Options:**

* `-o <DIR>`, `--output-dir <DIR>`: The base directory where files and directories will be created or deleted.
  * **Default:** `./project-generated`. This path is relative to the **current working directory** where you run the command.
  * The directory will be created if it doesn't exist.
  * The command will fail if the specified path exists but is not a directory.
* `-f`, `--force`: Overwrite existing files when a `File` action targets a path that already exists. Without this flag, existing files will be skipped.
* `-h`, `--help`: Print help information.
* `-V`, `--version`: Print version information.

## Input Markdown Format

The processor identifies actions based on specific header patterns immediately preceding a fenced code block (for `File` actions) or standing alone (for `Deleted File` actions).

### Action Headers

The following header formats are recognized:

**1. `File` Actions (Create/Overwrite):**

These headers must be immediately followed by a fenced code block (e.g., `````, ```` ``` ````). The content of the code block becomes the content of the file.

* **Markdown Headers:**
  * `## File: path/to/your/file.txt`
  * `**File: path/to/your/file.txt**`
* **Backtick Path Only:** (Implies `File` action)
  * `` `path/to/your/file.txt` ``
  * `1.`path/to/your/file.txt`` (Numbered list item)
  * `**`path/to/your/file.txt`**`
  * `##`path/to/your/file.txt``

**Example (Standard Header):**

````markdown
## File: src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```
````

**Example (Backtick Path):**

````markdown
`config.toml`

```toml
[settings]
enabled = true
```
````

**2. `Deleted File` Actions:**

These headers define files to be deleted.

* **Standalone Headers:** These headers should *not* be followed by a code block.
  * `## Deleted File: path/to/old/file.log`
  * `**Deleted File: path/to/old/file.log**`

* **Special Case (Path in Block):** A `## Deleted File:` header *without* a path, immediately followed by a code block containing *only* the path to delete on its first line.

````markdown
## Deleted File:

```text
path/inside/block_to_delete.tmp
```
````

**3. Internal Comment Headers (Inside Code Blocks):**

These headers can appear on the *first line* inside a code block to define the file path.

* `// File: path/to/file.ext`: The header line itself is **excluded** from the file content. Supports paths in backticks (`// File:`path with spaces.txt``).

    ```javascript
    // File: utils/helper.js
    function greet(name) {
      console.log(`Hello, ${name}!`);
    }
    module.exports = { greet };
    ```

    *(Resulting `utils/helper.js` will not contain the `// File:` line)*

* `// path/to/file.ext`: The header line is **included** in the file content. This is useful for self-documenting scripts.

    ```javascript
    // scripts/run_analysis.js
    console.log("Running analysis...")
    ... rest of script
    ```

    *(Resulting `scripts/run_analysis.js` will contain the `// scripts/...` line)*

    *Heuristics:* To avoid misinterpreting actual comments or strings as headers:
  * Lines starting `//` (with a space) are only treated as paths if they contain typical path characters (`/`, `\`, `.`). Otherwise, they are treated as regular comments.
  * Lines matching comment patterns (`#`, `--`, `;`, etc.) or simple string literal patterns (`"..."`, `'...'`, `` `...` ``) are ignored even if they match a header format internally.

**4. Wrapped Headers:**

A header can be placed inside a ` ```markdown ` or ` ```md ` block, and it will apply to the *next adjacent* code block (only whitespace allowed between the blocks). This is useful for complex content or when the header itself contains characters that interfere with markdown.

* **Create:**

    ````markdown
    ## File: complex_config.yaml

    ```yaml
    # This is the actual content
    settings:
      feature_a: true
      nested:
        value: 123
    ```
    ````

* **Delete:**

    ````markdown
    **Deleted File: legacy_script.sh**

    *(No following code block needed for delete)*
    ````

### Path Handling and Safety

* Paths specified in headers are treated as relative to the `--output-dir`.
* Parent directories are created automatically as needed.
* **Safety:** The tool prevents writing outside the resolved base output directory. Paths containing `..` that would escape the base directory will cause the action to fail safely.
* Paths containing invalid components (like `//` or trailing `/`) will be skipped.

### Content Handling

* The *entire* content within the fenced code block (excluding the fences themselves) is written to the file.
* A trailing newline (`\n`) is added to the file content if it doesn't already end with one. This ensures POSIX-compatible text files.

## Examples

**Input (`example.md`):**

````markdown
# Example Project Structure

This file defines a simple project.

## File: src/main.py

```python
# Main application script
# // File: src/utils.py (This is just a comment, ignored)
import utils

def main():
    print("Starting app...")
    utils.helper()
    print("App finished.")

if __name__ == "__main__":
    main()
```

## File: src/utils.py

```python
# Utility functions
def helper():
    print("Helper function called.")

```

**Deleted File: old_data.csv**

`README.md`

```markdown
# My Project

Generated by Strux.
```

```md
## Deleted File: temp/to_delete.log
```

```markdown
// File: .gitignore
*.pyc
__pycache__/
```

````

**Command:**

```bash
# Assuming you run this in /home/user/projects/
strux -o my_project example.md
# Output will be in /home/user/projects/my_project
```

**Result (`my_project` directory):**

```plaintext
my_project/
├── .gitignore
├── README.md
└── src/
    ├── main.py
    └── utils.py
```

* `my_project/src/main.py` contains the Python code (without the `// File:` comment line).
* `my_project/src/utils.py` contains its Python code.
* `my_project/README.md` contains `# My Project\n\nGenerated by Strux.\n`.
* `my_project/.gitignore` contains `*.pyc\n__pycache__/\n`.
* Any pre-existing `my_project/old_data.csv` or `my_project/temp/to_delete.log` would be deleted.

## Development

### Prerequisites

* Rust & Cargo ([https://rustup.rs/](https://rustup.rs/))
* `pre-commit` ([https://pre-commit.com/](https://pre-commit.com/))

    ```bash
    pip install pre-commit
    # or: brew install pre-commit
    ```

### Setup

1. Clone the repository.
2. Install pre-commit hooks:

    ```bash
    pre-commit install
    ```

    This will ensure that formatting (`cargo fmt`), linting (`cargo clippy`), tests (`cargo test`), and other checks run automatically before each commit.

### Building

```bash
cargo build         # Development build
cargo build --release # Release build
```

### Testing

```bash
cargo test          # Run all tests (unit, integration, documentation)
```

### Pre-commit Hooks

This project uses `pre-commit` to automatically run code quality checks and apply fixes (formatting, linting, tests) before each commit. This helps maintain code consistency and catch issues early.

**Usage:**

Once installed (`pre-commit install`), `pre-commit` will run automatically when you run `git commit`.

* **Automatic Fixes:** Hooks for `cargo fmt`, `cargo fix`, and `cargo clippy --fix` will attempt to automatically fix formatting issues, compiler suggestions, and simple lints.
* **Commit Flow:**
    1. You run `git commit`.
    2. `pre-commit` runs the hooks.
    3. If any fixing hook modifies files (e.g., applies formatting), the commit will be **aborted**.
    4. You will see messages indicating which files were changed. **Review the changes** and use `git add <modified files>` to stage them.
    5. Run `git commit` **again**.
    6. This time, the fixing hooks should find nothing to change. The checking hooks (`clippy`'s check part, `cargo test`) will then run.
    7. If all checks pass, the commit succeeds.
* **Manual Fixes:** If `cargo clippy` or `cargo test` fail after the automatic fixing stage, you will need to manually fix the reported errors, `git add` your changes, and commit again.

**Manual Run:**

You can run all checks and fixes manually on all files at any time:

```bash
pre-commit run --all-files
```

**Skipping Checks (Use with Caution):**

If you need to bypass the pre-commit checks for a specific commit (e.g., work-in-progress), you can use the `--no-verify` flag:

```bash
git commit --no-verify -m "Your commit message"
```

## Commit Messages

This project follows the [Conventional Commits specification](https://www.conventionalcommits.org/). Please adhere to these guidelines when contributing. See [COMMIT.md](COMMIT.md) for detailed rules and examples specific to this project.
