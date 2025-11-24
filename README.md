# Kanayago for Zed

A Zed extension that enables [Kanayago](https://github.com/S-H-GAMELINKS/kanayago) as a Language Server Protocol (LSP) backend for Ruby.

## What is Kanayago?

Kanayago is a Ruby library that exposes the internal AST produced by Ruby's parser (`parse.y`) and provides a Language Server Protocol (LSP) implementation.
It offers syntax checking and AST-based analysis for Ruby code.

## Features

- Ruby syntax checking powered by Kanayago
- AST-based Ruby code analysis
- Lightweight LSP implementation fully written in Ruby

## Requirements

Before installing this extension, make sure the following is available:

- **`kanayago` command available in your PATH**

You can install Kanayago via RubyGems:

``` bash
gem install kanayago
```

## Installation

1.  Open Zed.
2.  Open the command palette:
    -   `Cmd+Shift+P` (macOS)
    -   `Ctrl+Shift+P` (Linux / Windows)
3.  Search for **"Extensions"** and open the Extensions view.
4.  Search for **"Kanayago"** and install the extension.

## Usage

After installation, the Kanayago LSP starts automatically when you open a Ruby (`.rb`) file.

No additional configuration is required as long as the `kanayago` command is in your PATH.

## License

This project is licensed under the MIT License.
See the [LICENSE](./LICENSE) file for details.

## Repository

https://github.com/S-H-GAMELINKS/zed-kanayago

