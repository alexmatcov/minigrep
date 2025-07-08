# minigrep

A simple command-line text search tool built in Rust, following [Chapter 12 of "The Rust Programming Language"](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) book.

## Features

- Search for text patterns in files
- Case-sensitive and case-insensitive search through:
  - Environment variable support (`IGNORE_CASE`)
  - Command-line flags (`-i`, `-s`)

## Installation

```bash
git clone https://github.com/alexmatcov/minigrep.git
cd minigrep
cargo build --release
```

## Usage

```bash
# change directory to the executable
cd src/target/release/

# run
./minigrep <query> <file_path> [flags]
```

where:
- `minigrep` is the executable binary found in `src/target/release/minigrep`;

- `<query>` is the string pattern you're searching for;
  
- `<file_path>` is the path to your document;
  
- *optional:* `[flags]` are the command-line arguments for:
  - `-s` case-sensitive search
  - `-i` case-insensitive search
  
  **NOTE**: Default configuration (no set flags) is case-sensitive.

## Examples
### Basic search
```bash
./minigrep to poem.txt
```

### Case-insensitive search
```bash
./minigrep rUst poem.txt -i
```

### Using environment variable
```bash
IGNORE_CASE=1 ./minigrep to poem.txt
```
The last example sets an environment variable to use the case-insensitive configuration.

In case of setting both the environment variable and the command-line argument the latter will take precedence.

## Development
```bash
cargo run -- <query> <file_path> [flags]
```

## Learning Concepts
The project demonstrates key Rust concepts:

- **Data structures: Vectors and Strings**

- **Error handling with Result<T, E>**

- **Lifetime management**

- **Using trais where appropriate**
    
- **Unit testing**
  
- **Command-line argument parsing**

- **Environment variables**