# ono
ono is a cross-platform  CLI tool for managing command snippets. It saves command snippets in markdown format so you can easily browse or manage your command snippets with tools like Obsidian, or use version controller to take a backup.

[![Rust](https://github.com/arunes/ono-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/arunes/ono-rust/actions/workflows/rust.yml)

- [[#Configuration]]
- [[#Snippet Format]]
- [[#Command Reference]]

---
## Configuration
On startup, ono will try to read the configuration file from your OS's default config directory.

| Platform | Value                             | Example                                               |     |
| -------- | --------------------------------- | ----------------------------------------------------- | --- |
| Linux    | $XDG_CONFIG_HOME or $HOME/.config | /home/alice/.config/ono/ono.yaml                      |     |
| macOS    | $HOME/Library/Application Support | /Users/Alice/Library/Application Support/ono/ono.yaml |     |
| Windows  | {FOLDERID_RoamingAppData}         | C:\Users\Alice\AppData\Roaming/ono/ono.yaml           |     |

If no config file present, ono will try to determine the config values automatically. 

| Config Key   | Description                         |     |
| ------------ | ----------------------------------- | --- |
| data_dir     | Location of command snippets        |     |
| history_file | Location of your shell history file |     |
| editor       | Your default text editor            |     |
To check what ono automatically set for config values you can run `ono config` command.

---
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

> At least one code block is required in markdown for fsnippet to show in ono.

Fields
- `description` Used for fuzzy search along with the snippet
- `used` Number of times snippet is used. Affects search result ordering, most used snippets shows higher.
- `last_used` Last use date of the snippet. Affects search result ordering, recently used snippets shows higher.
- `alias` Shortcut to run retrieve snippet with `ono alias` 

---
## Command Reference
| Command  | Description                                                                                                                                                                                         | Example           |                                                            |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------- | ---------------------------------------------------------- |
| `hist`   | Shows command picker using the shell history, and uses the selected command to populate new snippet form.                                                                                           | `ono hist`        | Command field will be populated with the selected command. |
| `clip`   | Uses the system clipboard to populate new snippet form.                                                                                                                                             | `ono clip`        | Command field will be populated with the clipboard content |
| `add`    | Populates new snippet form with the command specified after `add`.<br><br>> If your shell supports double bang you can use `ono add !!` Instead of typing the command to add last executed command. | `ono add "df -h"` | Command field will be populated with `df -h`               |
| `config` | Shows the current configuration.                                                                                                                                                                    | `ono config`      |                                                            |
| `*`      | Everything else other than the commands above will be treated as aliases.                                                                                                                           | `ono list`        | Retrieves the snippet with `list` alias.                   |

