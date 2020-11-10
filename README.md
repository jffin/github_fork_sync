# GitHub Fork Sync

Simple command-line tool to fork a GitHub repository with a token.

## Download Binary
You can download last binary on [releases]() page

## Build From Source
```
# download the code
$ git clone https://github.com/jffin/github_fork_sync.git
$ cd github_fork_sync.git

# Build in release mode
$ cargo build --release

# run the binary
./target/release/github_fork_sync --help
```

## Usage
```.env
Fork GitHub repository from a terminal.

USAGE:
    github_fork_sync [OPTIONS] [TARGET_REPOSITORY]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --github-token <token>    GitHub token

ARGS:
    <TARGET_REPOSITORY>    The repository you want to fork
```

## Output
Success message with new URL of the forked repository
