# GitHub Stars Fetcher

A simple command-line utility that fetches and displays star counts for GitHub repositories.

## Features

- Accepts a list of GitHub repositories from stdin
- Fetches star counts for each repository using the GitHub API
- Displays results in a formatted table, sorted by star count (descending)
- Handles API rate limits with appropriate delays

## Installation

```bash
# Clone the repository
git clone https://github.com/tmartinfr/github-stars-fetcher
cd github-stars-fetcher

# Install the project
cargo install --path .
```

## Usage

```bash
# Basic usage with pipe
echo "rust-lang/rust
microsoft/vscode
facebook/react" >repository_list.txt

./github-stars-fetcher <repository_list.txt
```

## Example Output

```
+-------------------+--------+--------------------------------------+
| Repository        | Stars  | URL                                  |
+-------------------+--------+--------------------------------------+
| rust-lang/rust    | 85421  | https://github.com/rust-lang/rust    |
| facebook/react    | 207632 | https://github.com/facebook/react    |
| microsoft/vscode  | 147023 | https://github.com/microsoft/vscode  |
+-------------------+--------+--------------------------------------+
```

## Configuration

To avoid GitHub API rate limits, you can use a GitHub personal access token.

Set the `GITHUB_TOKEN` environment variable:

```bash
export GITHUB_TOKEN=your_github_token
```

Using a GitHub token is optional but recommended to avoid rate limiting on frequent API calls.

## Dependencies

- `prettytable-rs`: For formatted table output
- `reqwest`: For HTTP requests
- `serde`: For JSON deserialization
- `tokio`: For async runtime

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

