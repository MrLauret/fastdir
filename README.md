# fastdir

A simple command-line tool for saving, managing, and quickly returning to directories by name.

Instead of remembering long paths, register directories once and jump to them instantly from your shell.

---

## Features

- Add directories with a custom name
- Delete saved directories
- List all saved directories
- Return a directory path by name (for shell `cd` usage)
- Stores data locally in a JSON file in your home directory

---

## Installation

### Build from source

```bash
git clone https://github.com/your-username/fastdir
cd fastdir
cargo build --release

Binary output:

target/release/fastdir

(Optional) move it into your PATH:

sudo mv target/release/fastdir /usr/local/bin/
Usage
Add a directory
fastdir add /path/to/project myproject
Delete a directory
fastdir delete myproject
List directories
fastdir list

Example output:

Your directories:

myproject: /path/to/project
backend: /home/user/backend
Return a directory (for cd)

Prints the stored path:

fastdir return myproject

Use it with cd:

cd "$(fastdir return myproject)"

Or make a helper function:

d() {
    cd "$(fastdir return "$1")"
}

Then:

d myproject
Data storage

Directories are stored in:

~/dirs.json

Example format:

[
  {
    "name": "myproject",
    "path": "/home/user/projects/myproject"
  }
]
Project structure
src/
├── main.rs
├── cli.rs
├── store.rs
main.rs → CLI routing
cli.rs → command definitions (clap)
store.rs → storage + logic
Why this exists

A lightweight alternative to directory jump tools like zoxide, focused on simplicity and full control.