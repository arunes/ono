**_This project is being under active development and its not stable at the moment._**


# ono
ono is a cross-platform  CLI tool for managing command snippets. It saves command snippets in markdown format so you can easily browse or manage your command snippets with tools like Obsidian, or use version controller to take a backup.

[![Rust](https://github.com/arunes/ono-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/arunes/ono-rust/actions/workflows/rust.yml)

- [Installation](#installation)
- [Getting Started](#getting-started)
- [Configuration](#configuration)
- [Adding Snippets](#adding-snippets)
- [Snippet Format](#snippet-format)
- [Command Reference](#command-reference)

<br/>

## Installation
TODO!

### Build from source
TODO!

### Install binaries
TODO!

<br/>

## Getting Started
After you install `ono` executable will be available in the terminal. Executing `ono` without any argument will open the main TUI. In the main app you can,

- Add snippets
- Edit snippets
- Delete snippets
- Set aliases to snippets

For more advanced usage, check [Adding Snippets](#adding-snippets), or [Command Reference](#command-reference).

<br/>

## Configuration
Configuration is done through ono.yaml file. If no config file present, ono will try to determine the config values automatically.

> To check what ono automatically set for config values you can run `ono config` command.

### Sample configuration
<details>
<summary><strong>Preview example configuration file</strong></summary>
<br>
  
```yaml
data_dir: /home/alice/.local/share/ono
editor: /user/bin/nvim
history_file: /home/alice/.bash_history
```
</details>

### Configuration keys

| Config Key   | Description                         |
| ------------ | ----------------------------------- |
| data_dir     | Location of command snippets        |
| history_file | Location of your shell history file |
| editor       | Your default text editor            |

### Configuration File Location
| Platform | Value                             | Example                                               |     |
| -------- | --------------------------------- | ----------------------------------------------------- | --- |
| Linux    | $XDG_CONFIG_HOME or $HOME/.config | /home/alice/.config/ono/ono.yaml                      |     |
| macOS    | $HOME/Library/Application Support | /Users/Alice/Library/Application Support/ono/ono.yaml |     |
| Windows  | {FOLDERID_RoamingAppData}         | C:\Users\Alice\AppData\Roaming\ono\ono.yaml           |     |

<br/>

## Adding Snippets
You can use couple of different ways to add snippets to ono. 

- `ono hist` Add snippet from history file
  This command will show you your shell history and let you pick one to add.
  
- `ono clip` Add snippet from clipboard
  This command will read your clipboard content and set that as a snippet.

- `ono add "command"` Add snipped manually
  This command will use the input after `add` and set that as a snippet.
  > If your shell supports double bang you can use `ono add !!` Instead of typing the command to add last executed command.

- Or you can use any text editor to create a md file following the snippet format below, in the `data_dir` directory. 

<br/>

## Managing Snippets
Running `ono` without any arguments will run main app. You can search, delete, edit, and set aliases.

<br/>

## Snippet Format
ono will use the below markdown template to create snippets. 

~~~
---
description: List directory with details and colors
used: 5
last_used: 2025-12-31 17:00
alias: ls
---

```bash
ls -alFh --time-style=long-iso --color=auto
``
~~~

> At least one code block is required in markdown for snippet to show in ono.

Fields
- `description` Used for fuzzy search along with the snippet
- `used` Number of times snippet is used. Affects search result ordering, most used snippets shows higher.
- `last_used` Last use date of the snippet. Affects search result ordering, recently used snippets shows higher.
- `alias` Shortcut to run retrieve snippet with `ono alias` 

<br/>

## Command Reference
| Command  | Description                                                                                                                                                                                         | Example           |                                                            |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------- | ---------------------------------------------------------- |
| `hist`   | Shows command picker using the shell history, and uses the selected command to populate new snippet form.                                                                                           | `ono hist`        | Command field will be populated with the selected command. |
| `clip`   | Uses the system clipboard to populate new snippet form.                                                                                                                                             | `ono clip`        | Command field will be populated with the clipboard content |
| `add`    | Populates new snippet form with the command specified after `add`. | `ono add "df -h"` | Command field will be populated with `df -h`               |
| `config` | Shows the current configuration.                                                                                                                                                                    | `ono config`      |                                                            |
| `*`      | Everything else other than the commands above will be treated as aliases.                                                                                                                           | `ono list`        | Retrieves the snippet with `list` alias.                   |

