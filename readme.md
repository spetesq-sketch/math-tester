# math-tester

A terminal-based mental math trainer written in Rust. Practice arithmetic operations, track your response times,
and save your overall progress

![Rust](https://img.shields.io/badge/rust-2024-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

---

## Features

- **Customizable Practice**: Configure custom number domains, operation types, decimal precision, and example
  counts
- **Precision Timing**: Measures your response time for each problem as well as your overall session duration.
- **Persistent Statistics**: Saves your lifetime statistics (total examples solved, correct/wrong ratio, total time
  spent) locally in JSON format.
- **CLI Subcommands**: Command-line interface powered by `clap`

---

## Usage

### Start a Game Session:

```bash
math-tester game
```

### Get your Statistic:

```bash
math-tester stat
```

## Installation

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/spetesq-sketch/math-tester.git
cd math-cli

# Build the release binary
cargo build --release
```

## Data Storage

The data located:

Linux: `~/.local/share/math_thing/data.json`

macOS: `~/Library/Application Support/math_thing/data.json`

Windows: `C:\Users\<User>\AppData\Local\math_thing\data.json`
