# Strux

`strux` is a command-line utility written in Rust that parses specially formatted markdown files to generate directory structures and file contents. It's useful for:

* **Project Bootstrapping:** Quickly create initial project layouts from a template.
* **Documentation:** Document file structures and provide their content within a single markdown file.
* **Reproducible Setups:** Define file layouts declaratively.

The tool reads a markdown file, identifies actions (like creating, deleting, or moving files) based on specific header formats, and executes these actions relative to a specified output directory.

## Features

* Supports creating files with content defined in code blocks.
* Supports deleting files.
* Supports moving files.
* Multiple header formats for defining actions (Markdown headers, backticks, internal comments).
* "Wrapped" header format for associating headers with subsequent code blocks or for standalone delete/move actions.
* Automatic creation of parent directories for created or moved files.
* Safety checks to prevent writing or moving outside the target base directory.
* Option to force overwriting existing files (for create and move actions).
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

* `-o <DIR>`, `--output-dir <DIR>`: The base directory where files and directories will be created, deleted or moved.
  * **Default:** `./project-generated`. This path is relative to the **current working directory** where you run the command.
  * The directory will be created if it doesn't exist.
  * The command will fail if the specified path exists but is not a directory.
* `-f`, `--force`: Overwrite existing files when a `File` or `Moved File` action targets a path that already exists as a file. Without this flag, existing files will be skipped. This flag does not allow replacing a directory with a file.
* `-h`, `--help`: Print help information.
* `-V`, `--version`: Print version information.

## Input Markdown Format

The processor identifies actions based on specific header patterns.

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

**2. `Deleted File` Actions:**

These headers define files to be deleted. They should *not* be followed by a code block unless using the special case below.

* **Standalone Headers:**
  * `## Deleted File: path/to/old/file.log`
  * `**Deleted File: path/to/old/file.log**`

* **Special Case (Path in Block):** A `## Deleted File:` header *without* a path, immediately followed by a code block containing *only* the path to delete on its first line.

````markdown
## Deleted File:

```text
path/inside/block_to_delete.tmp
```
````

**3. `Moved File` Actions:**

These headers define files to be moved. They should *not* be followed by a code block. The keyword " to " (case-sensitive, with spaces) separates the source and destination paths.

* **Standalone Headers:**
  * `## Moved File: old/path/file.txt to new/path/file.txt`
  * `**Moved File: old/path/file.txt to new/path/file.txt**`
* If a path segment itself contains " to ", that segment must be enclosed in backticks:
  * `## Moved File: \`archive/file with to in name.log\` to archive/renamed_file.log`
  * `## Moved File: old/path.txt to \`new path with to in name.txt\``

**Example (Moved File):**
````markdown
## Moved File: temp/report.docx to final/official_report.docx
````

**4. Internal Comment Headers (Inside Code Blocks for `File` actions):**

These headers can appear on the *first line* inside a code block to define the file path for a `File` action.

* `// File: path/to/file.ext`: The header line itself is **excluded** from the file content. Supports paths in backticks (`// File:\`path with spaces.txt\``).

    ```javascript
    // File: utils/helper.js
    function greet(name) {
      console.log(`Hello, ${name}!`);
    }
    module.exports = { greet };
    ```

* `// path/to/file.ext`: The header line is **included** in the file content.

    ```javascript
    // scripts/run_analysis.js
    console.log("Running analysis...")
    ```

    *Heuristics apply to avoid misinterpreting comments as paths.*

**5. Wrapped Headers:**

A header can be placed inside a ` ```markdown ` or ` ```md ` block.
* For `File` actions, it applies to the *next adjacent* code block.
* For `Deleted File` or `Moved File` actions, it's a standalone action.

* **Create Example:**

    ````markdown
    ```markdown
    ## File: complex_config.yaml
    ```

    ```yaml
    # This is the actual content
    settings:
      feature_a: true
    ```
    ````

* **Delete Example:**

    ````markdown
    ```markdown
    **Deleted File: legacy_script.sh**
    ```
    *(No following code block needed for delete)*
    ````

* **Move Example:**
    ````markdown
    ```markdown
    ## Moved File: staging/data.csv to processed/data.csv
    ```
    *(No following code block needed for move)*
    ````

### Path Handling and Safety

* Paths specified in headers are treated as relative to the `--output-dir`.
* Parent directories are created automatically as needed for `File` actions and for the destination of `Moved File` actions.
* **Safety:** The tool prevents writing or moving files outside the resolved base output directory. Paths containing `..` that would escape the base directory will cause the action to fail safely.
* Paths containing invalid components (like `//` or trailing `/`) will be skipped.

### Content Handling (for `File` actions)

* The *entire* content within the fenced code block (excluding the fences themselves and certain internal headers) is written to the file.
* A trailing newline (`\n`) is added to the file content if it doesn't already end with one.

## Examples

**Input (`example.md`):**

````markdown
# Example Project Structure

## File: src/main.py
```python
# Main application script
import utils
def main(): utils.helper()
if __name__ == "__main__": main()
```

## File: src/utils.py
```python
# Utility functions
def helper(): print("Helper function called.")
```

## File: temp/draft.txt
```
This is a draft file that will be moved.
```

**Deleted File: old_data.csv**

`README.md`
```markdown
# My Project
Generated by Strux.
```

## Moved File: temp/draft.txt to docs/final_draft.txt

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
├── docs/
│   └── final_draft.txt  # Moved from temp/draft.txt
└── src/
    ├── main.py
    └── utils.py
```

* `my_project/src/main.py` and `my_project/src/utils.py` contain their Python code.
* `my_project/README.md` contains its content.
* `my_project/.gitignore` contains its content.
* `my_project/docs/final_draft.txt` contains "This is a draft file that will be moved.\n" (moved from `my_project/temp/draft.txt`).
* The original `my_project/temp/draft.txt` is gone as it was moved.
* Any pre-existing `my_project/old_data.csv` or `my_project/temp/to_delete.log` (if they existed in the output directory before the run) would be deleted.

## Development

### Prerequisites

* Rust & Cargo ([https://rustup.rs/](https://rustup.rs/))
* `pre-commit` ([https://pre-commit.com/](https://pre-commit.com/))

### Setup

1. Clone the repository.
2. Install pre-commit hooks: `pre-commit install`

### Building

```bash
cargo build         # Development build
cargo build --release # Release build
```

### Testing

```bash
cargo test          # Run all tests
```

### Pre-commit Hooks

This project uses `pre-commit` for automated code quality checks (formatting, linting, tests) before each commit.

## Commit Messages

This project follows the [Conventional Commits specification](https://www.conventionalcommits.org/). See [COMMIT.md](COMMIT.md) for details.
