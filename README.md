# GitHub Activity CLI Tool

[![Build Status](https://github.com/seabensea/github-activity/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/seabensea/github-activity/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/seabensea/github-activity)](https://github.com/seabensea/github-activity/releases)

A simple command line interface (CLI) to fetch the recent activity of a GitHub user and display it in the terminal. This tool is built using Rust and provides a local testing server to simulate GitHub events.

## Features

- Fetch GitHub activity events for a specific user or globally.
- Display detailed information about various GitHub event types.
- Local testing server to simulate GitHub events.

## Prerequisites

- Rust (latest stable version recommended)
- Python 3.x
- Flask (for the local testing server)

## Installation

### From Pre-built Binaries

Pre-built binaries are available for **Windows**, **Linux**, and **macOS** on the [Releases](https://github.com/seabensea/github-activity/releases) page.

#### Windows

1. **Download the binary**:

   - Go to the [Releases](https://github.com/seabensea/github-activity/releases) page.
   - Download the `github-activity-release.zip` file.
   - Extract the contents of the ZIP file.

2. **Navigate to the download location** in Command Prompt or PowerShell:

   ```cmd
   cd C:\Path\To\Extracted\Folder\github-activity-windows
   ```

3. **Run the application** directly:

   ```cmd
   github-activity.exe
   ```

   You can now use the CLI by prefixing commands with `github-activity.exe`. For example:

   ```cmd
   github-activity.exe --username "seabensea"
   ```

#### Linux and macOS

1. **Download the binary**:

   - Go to the [Releases](https://github.com/seabensea/github-activity/releases) page.
   - Download the `github-activity-release.zip` file.
   - Extract the contents of the ZIP file.

2. **Navigate to the download location** in your terminal:

   ```bash
   cd /path/to/extracted/folder/github-activity-linux
   ```

3. **Make the binary executable**:

   ```bash
   chmod +x github-activity
   ```

4. **Run the application** directly:

   ```bash
   ./github-activity --help
   ```

   You can now use the CLI by prefixing commands with `./github-activity`. For example:

   ```bash
   ./github-activity --username "seabensea"
   ```

#### Advanced: Adding to PATH

For convenience, you can add the binary to a directory that's in your system's `PATH`, allowing you to run `github-activity` from anywhere without specifying the path.

**Windows:**

1. **Move the binary** to a directory in your `PATH`:

   - Copy or move the `.exe` file to `C:\Windows\System32` or any directory that's in your `PATH`.
   - Alternatively, add the directory containing `github-activity.exe` to your `PATH` environment variable.

2. **Verify installation**:

   Open a new Command Prompt or PowerShell window and run:

   ```cmd
   github-activity --help
   ```

**Linux and macOS:**

1. **Move the binary** to `/usr/local/bin/`:

   ```bash
   sudo mv github-activity /usr/local/bin/github-activity
   ```

2. **Verify installation**:

   Open a new terminal window and run:

   ```bash
   github-activity --help
   ```

### From Source

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/seabensea/github-activity.git
cd github-activity
```

Build the project in release mode:

```bash
cargo build --release
```

Run the project:

```bash
./target/release/github-activity --help
```

## Usage

### Fetch GitHub Events

Run the CLI tool to fetch GitHub events:

```bash
./target/release/github-activity --username <GitHub-username>
```

To fetch global events:

```bash
./target/release/github-activity
```

### Use Local Testing Server

1. Start the local testing server:

   ```bash
   python server.py
   ```

2. Run the CLI tool with the `--local` flag:

   ```bash
   ./target/release/github-activity --local
   ```

This will fetch events from the local server running at `http://localhost:5000/events`.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Credits

This project is inspired by the [GitHub User Activity](https://roadmap.sh/projects/github-user-activity) on [roadmap.sh](https://roadmap.sh/).
