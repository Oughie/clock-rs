# clock-rs

A modern, digital clock that _effortlessly_ runs in your terminal.

[![crates.io](https://img.shields.io/crates/v/clock-rs.svg)](https://crates.io/crates/clock-rs)
[![License](https://img.shields.io/github/license/Oughie/clock-rs)](LICENSE)
[![Stars](https://img.shields.io/github/stars/Oughie/clock-rs)](https://github.com/Oughie/clock-rs/stargazers)

![presentation](public/preview.png)

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
  - [Fields](#fields)
  - [Example](#example)
- [Contributing](#contributing)
- [License](#license)

## Introduction

`clock-rs` is a terminal-based clock written in Rust, designed to be a new alternative to [tty-clock](https://github.com/xorg62/tty-clock). It offers several improvements, which include:
- The use of a single configuration file to manage its settings, with the ability to overwrite them through the command line,
- Many additional features such as a timer, stopwatch, and greater flexibility,
- And the active development that will bring many updates in the near future!

## Installation

To install `clock-rs` globally using Cargo, simply run `$ cargo install clock-rs`.
You can then run the executable via the `clock-rs` command.

## Usage

```
Usage: clock-rs [OPTIONS] [COMMAND]

Commands:
  clock      Display the current time (default)
  timer      Create a timer
  stopwatch  Start a stopwatch
  help       Print this message or the help of the given subcommand(s)

Options:
  -c, --color <COLOR>        Specify the clock color
  -x, --x-pos <X_POS>        Set the position along the horizontal axis
  -y, --y-pos <Y_POS>        Set the position along the vertical axis
      --fmt <FMT>            Set the date format
  -t                         Use the 12h format
  -i, --interval <INTERVAL>  Set the polling interval in milliseconds
      --utc                  Use UTC time
  -s, --hide-seconds         Do not show seconds
  -B, --blink                Set the colon to blink
  -b, --bold                 Use bold text
  -h, --help                 Print help
  -V, --version              Print version
```

If no command is specified, the `clock` command is used by default.  
Thus, running `$ clock-rs clock` or simply just `$ clock-rs`, will both display the current time.

To exit the application, press either `Escape`, `q`, or `Ctrl-C`.

## Configuration

`clock-rs` uses the [TOML](https://toml.io/en/) file format for its settings.  
By default, the configuration file is named `conf.toml` and is stored in the OS configuration directory, within the `clock-rs` subdirectory.

On Linux: `~/.config/clock-rs/conf.toml`

This path can be changed using the `CONF_PATH` environment variable.  
Any argument passed in the command line will overwrite these settings.

### Fields

Here's a list of the available fields inside the `conf.toml` file.

| Field                     | Description                                | Possible values                                                                                                                   | Default      |
| ------------------------- | -------------------------------------------| ----------------------------------------------------------------------------------------------------------------------------------| ------------ |
| `general.color`           | Specify the color of the clock             | `"black"`, `"red"`, `"green"`, `"yellow"`, `"blue"`, `"magenta"`, `"cyan"`, or `"white"`. Optionally, prefix them with `bright-`. | `"white"`    |
| `general.interval`        | Set the polling interval in milliseconds   | Any Unsigned Integer, e.g. `499`.                                                                                                 | `500`        |
| `general.blink`           | Set the colon to blink                     | `true` or `false`.                                                                                                                | `false`      |
| `general.bold`            | Use bold text                              | `true` or `false`.                                                                                                                | `false`      |
| `position.horizontal`     | Set the position along the horizontal axis | `"start"`, `"center"`, or `"end"`.                                                                                                | `"center"`   |
| `position.vertical`       | Set the position along the vertical axis   | `"start"`, `"center"`, or `"end"`.                                                                                                | `"center"`   |
| `date.fmt`                | Specify the date format                    | Any String, e.g. `%A, %B %d, %Y`.                                                                                                 | `"%d-%m-%Y"` |
| `date.use_12h`            | Use the 12h format                         | `true` or `false`.                                                                                                                | `false`      |
| `date.utc`                | Use UTC time                               | `true` or `false`.                                                                                                                | `false`      |
| `date.hide_seconds`       | Do not show seconds                        | `true` or `false`.                                                                                                                | `false`      |

### Example

The `conf.toml` file could look like this:

```toml
[general]
color = "magenta"
interval = 499
blink = true
bold = true

[position]
horizontal = "start"
vertical = "end"

[date]
fmt = "%A, %B %d, %Y"
use_12h = true
utc = true
hide_seconds = true
```

The default configuration can be found [here](public/default.toml).

## Contributing

Feel free to report bugs, suggest features or contribute code.  
Any help is appreciated!

## License

Copyright © 2024 Oughie

This repository is licensed under the Apache License 2.0 - See [LICENSE](LICENSE) for more information.
