# Bookmark Manager

A simple command-line bookmark manager written in Rust. This tool allows you to manage bookmarks by adding, listing, and initializing a bookmark file. It offers a basic but useful interface for managing your bookmarks locally in a `.txt` file.

## Features

- **Add a Bookmark**: Save a URL as a bookmark.
- **List Bookmarks**: Display all saved bookmarks.

## Installation

### Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.

### Clone the Repository

To get started, first clone the repository:

```bash
git clone https://github.com/anishpoudel007/bookmark-manager.git
cd bookmark-manager
```

## Build

To build the project, run the following command

```bash
cargo build --release
```

This will generate the executable bm in the target/release directory.

### Run the project

After building the project, you can use the `bm` command to interact with the bookmark manager.

## Usage

### Initialize Bookmark Manager

Initialize the bookmark manager by creating the bookmark file:

```bash
bm init
```

This command will create the `bm.txt` file in the current directory if it does not exist.

### Add a Bookmark

To add a new bookmark, use the create subcommand followed by the URL:

```bash
bm create https://example.com
```

This will add the URL `https://example.com` to the `bm.txt` file.

### List Bookmarks

To list all saved bookmarks, use the list subcommand:

```bash
bm list
```

This will print all the URLs saved in `bm.txt` to the console.
