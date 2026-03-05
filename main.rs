//! A CLI productivity tool to create file trees with ease and fast
//!
//! Pipeline:
//!     1. Parse query and construct a buffer
//!     2. Collect files and directories
//!     3. Create directories and files
use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process::exit;

const HELP: &str = r#"Usage: construct [OPTIONS] <QUERY>

For more information, try passing `--help`."#;

const ADVANCED_HELP: &str = r#"Create multiple files and directories with one command line query

Usage: construct [OPTIONS] <QUERY>

OPTIONS
-------

    --help   Display this help message


ARGUMENTS
---------

    <QUERY>  The query indicating what files or directories to create


    QUERY SYNTAX
    ------------

        NOTE: Every object is separated by a `/` (forward slash)
            To indicate what type of object to create, you use a special character
            in front of the object name that is then stripped away to ultimately create
            the desired object.
            Directories and files that already exist will be ignored.

        SPECIAL CHARACTERS
        ------------------
            `+` => Indicates to create a file.
            `~` => Indicates to create a subdirectory within the same relative current directory.
                   Alternatively, you can use `..` to move back directories.
            `` (NO CHARACTER) => Indicates to create a sub directory.


EXAMPLE
-------

    - construct directory/~b/~c/~d/+file1.txt/e/+file2.txt/+file3.rs/f/../g/+finalFile.txt

    TREE RESULT
    -----------

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
                |-  finalFile.txt"#;

fn normalize_str(string: String) -> String {
    string.replace("\\", "/")
}

/// Returns (Files, Dirs)
fn get_files_and_dirs(query: String) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut buffer: PathBuf = PathBuf::new();
    let mut files: Vec<PathBuf> = Vec::new();
    let mut dirs: Vec<PathBuf> = Vec::new();

    for frag in query.split("/") {
        if frag.is_empty() {
            continue;
        }
        match frag.chars().nth(0).unwrap() {
            '+' => {
                let mut buf = buffer.clone();
                buf.push(&frag[1..]);
                files.push(buf);
            }
            '~' => {
                let mut buf = buffer.clone();
                buf.push(&frag[1..]);
                dirs.push(buf);
            }
            _ => {
                buffer.push(frag);
                dirs.push(buffer.clone());
            }
        }
    }

    (files, dirs)
}

/// Given the files and directories wanted to be created,
/// directories are created first so that files can be created
/// assuring that their destinations exist
///
/// Returns a vector of objects created
fn summon(files: Vec<PathBuf>, dirs: Vec<PathBuf>) -> Result<()> {
    for dir in dirs {
        if dir.exists() {
            continue;
        }
        fs::create_dir(&dir)?;
    }

    for file in files {
        if file.exists() {
            continue;
        }
        fs::File::create(&file)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", HELP);
        exit(1);
    }
    if &args[1] == "--help" {
        eprintln!("{}", ADVANCED_HELP);
        exit(1);
    }
    let query = args[args.len() - 1].clone();

    let (files, dirs) = get_files_and_dirs(normalize_str(query));
    summon(files, dirs)?;
    Ok(())
}
