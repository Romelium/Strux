#

## Development

### Pre-commit Hooks

This project uses `pre-commit` to automatically run code quality checks and apply fixes (formatting, linting, tests) before each commit. This helps maintain code consistency and catch issues early.

**Setup:**

1. **Install pre-commit:**
    If you don't have it, install it using pip:

    ```bash
    pip install pre-commit
    # Or using brew on macOS:
    # brew install pre-commit
    ```

2. **Install the git hooks:**
    Run this command in the root directory of the repository:

    ```bash
    pre-commit install
    ```

**Usage:**

Once installed, `pre-commit` will run automatically when you run `git commit`.

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
