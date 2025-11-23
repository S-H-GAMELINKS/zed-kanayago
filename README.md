# zed-kanayago

A Zed extension that enables [Kanayago](https://github.com/S-H-GAMELINKS/kanayago) LSP functionality in the Zed editor.

## About Kanayago

Kanayago is a library that processes AST nodes provided by Ruby's parser at the Ruby layer. It includes LSP (Language Server Protocol) functionality and provides features such as syntax checking for Ruby code.

## Overview

This extension allows you to use Kanayago's LSP features in the Zed editor, providing support for Ruby code analysis including syntax checking.

## Prerequisites

- Zed editor installed
- `kanayago` command available in your PATH

## Installation

### Installing Kanayago

First, install Kanayago itself:

```bash
gem install kanayago
```

### Installing the Zed Extension

1. Open Zed editor
2. Open the command palette with `cmd+shift+p` (macOS) or `ctrl+shift+p` (Linux/Windows)
3. Type `zed: extensions` to open the extensions menu
4. Search for "Kanayago" and install it

## Usage

Once the extension is installed, Kanayago's LSP functionality will automatically activate when you open Ruby files (`.rb`).

## Features

- Ruby code syntax checking
- AST-based code analysis

## License

See the LICENSE file for details about this project's license.

## Repository

https://github.com/S-H-GAMELINKS/zed-kanayago
