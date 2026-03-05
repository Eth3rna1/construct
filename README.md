# construct

## About
`construct` is a lightweight CLI productivity tool written in Rust that allows you **to create complex file and directory trees with a single command**.

Instead of manually creating folders and files step-by-step, `construct` lets you describe the structure using a concise query syntax.

## Features

* Create multiple directories and files in one command

* Simple query-based syntax

* Safe - existing files and directories are ignored

* Fast - written in Rust with minimal dependencies

* Ideal for bootstrapping projects, scaffolding structures, and automation


## Installation
From Source
```cmd
git clone https://github.com/Eth3rna1/construct.git
cd construct
rustc main.rs -o construct
```

## Usage
```cmd
construct [OPTIONS] <QUERY>
```

> Use the `--help` flag to learn the query syntax

## Query Syntax

The `<QUERY>` describes the file tree you want to create.

> Objects are separated by a slash (`/`)

### Special Characters
|Prefix| Meaning|
|-|-|
|+ | Create a file|
|~ | Create a subdirectory relative to the current directory buffer |
| (none) | Create a subdirectory and move the buffer into it |
|.. |Move up one directory |

> **Existing files or directories are skipped automatically.**

### Example
```cmd
construct directory/~b/~c/~d/+file1.txt/e/+file2.txt/+file3.rs/f/../g/+finalFile.txt
```

Results to...
```text
\---directory
|-  file1.txt
    |
    +---b
    +---c
    +---d
    \---e
        |-  file2.txt
        |-  file3.rs
        |
        +---f
        \---g
            |-  finalFile.txt
```

## How It Works

The program follows a simple pipeline:

1. Parse query and normalize path separators

2. Collect files and directories

3. Create directories first

4. Create files

5. This ensures all file destinations exist before creation.

## Example Use Cases
* Project Scaffolding
```cmd
construct src/~components/~utils/+main.rs/+lib.rs/tests/+basic.rs
```

* Documentation Structure
```cmd
construct docs/~api/~guides/+getting-started.md/+installation.md
```

* Rapid Prototyping
```cmd
construct project/~src/+main.py/~tests/+test_main.py/+README.md
```

## Why Use construct?

Creating large folder structures often requires many repetitive commands like:

```cmd
mkdir -p src/components src/utils tests
touch src/main.rs src/lib.rs
```

With `construct`, this becomes:

```cmd
construct src/~components/~utils/+main.rs/+lib.rs/tests
```
