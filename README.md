# Codewars Kata Scraper

A command-line tool written in Rust that allows you to scrape and download kata exercises from Codewars. This tool can extract exercise details using either a kata URL or its unique code, saving the content locally for offline access.

## Features

- Download kata exercises using either a Codewars URL or kata code
- Automatic validation of kata codes and URLs
- Saves kata details in markdown format
- Creates exercise files with kata content
- Asynchronous operation for better performance
- Shows execution time for each operation

## Requirements

- Rust 1.70 or higher
- Internet connection to access Codewars

### Dependencies

```toml
[dependencies]
clap = { features = ["derive"] }
tokio = { features = ["full"] }
regex = "1.0"
```

## Installation

1. Clone the repository:
```bash
git clone [your-repository-url]
cd codewars-scraper
```

2. Build the project:
```bash
cargo build --release
```

The compiled binary will be available in `target/release/`.

## Usage

The scraper can be used in two ways:

### 1. Using a Kata URL

```bash
codewars-scraper --kata https://www.codewars.com/kata/57a4d500e298a7952100035d
```

### 2. Using a Kata Code Directly

```bash
codewars-scraper 57a4d500e298a7952100035d
```

Or with the explicit kata flag:
```bash
codewars-scraper --kata 57a4d500e298a7952100035d

```

## Command-Line Arguments

- `-k, --kata <URL/CODE>`: Accepts either a full Codewars kata URL or a 24-character kata code
- `<CODE>`: Direct input of a 24-character kata code
- `-h, --help`: Display help information
- `-V, --version`: Display version information

## Input Validation

The tool performs several validations:

1. Kata codes must be exactly 24 characters long
2. URLs must match the Codewars kata format: `https://www.codewars.com/kata/<code>`
3. Both the URL path and direct code input are validated using regex

## Output

The tool generates two files:

1. A markdown file containing kata details and metadata
2. A kata file containing the exercise content

## Error Handling

The tool provides clear error messages for common issues:

- Invalid kata codes or URLs
- Network connection problems
- Failed content retrieval
- Invalid input formats

## Development

The project uses several Rust crates:

- `clap`: For command-line argument parsing
- `tokio`: For async operations
- `regex`: For input validation
- Custom `codewars_scraping` module for core functionality

