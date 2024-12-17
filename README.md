# Monolog: A Note-Taking CLI Tool

Monolog is a simple command-line application for managing notes. It allows you to add notes, view today's notes, and retrieve the most recent notes.

## Features

- Add Notes: Add new notes quickly from the command line.
- Today's Notes: Display notes created today.
- Recent Notes: View a specific number of the most recent notes.

## Prerequisites

Ensure you have the following tools installed:

### 1. Rust and Cargo

Run `rustc --version` and `cargo --version` to confirm installation.

### 2. SQLite: SQLite is required for the database.

## Installation

Follow these steps to install and build monolog:

### 1. Clone the Repository

```bash
cargo install --git https://github.com/alexpetrean80/monolog.git
```

### 2. Install Dependencies

`monolog` uses the following dependencies:

`tokio`: For async runtime.
`sqlx`: For database interactions.
`chrono`: To handle date and time.
`clap`: To parse command-line arguments.

These dependencies are specified in Cargo.toml. To fetch and install them, run:

```bash
cargo fetch
```

### 3. Build the Application

Compile the project using Cargo:

```bash
cargo build --release
```

The binary will be available in the target/release directory.

## Usage

The `monolog` application provides the following subcommands:

### 1. Add a Note

To add a new note:

```bash
monolog add <your_note>
```

Example:

```bash
monolog add "This is my first note."
```

### 2. View Today's Notes

To display all notes added today:

```bash
monolog today
```

Example output:

```shell
# 2024-06-17

## 14:32
> This is my first note.

## 15:45
> Follow up on project status.
```

### 3. View Recent Notes

To display the n most recent notes:

```bash
monolog last <number_of_notes>
```

Example:

```bash
monolog last 5
```
