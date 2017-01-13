[![Build Status](https://travis-ci.org/flxo/rogcat.png)](https://travis-ci.org/flxo/rogcat)
# rogcat


A ``adb logcat`` wrapper with colors and filter and output options written in `Rust`

![Screenshot](/screenshot.png)

## Usage

```
rogcat 0.2.1
Felix Obenhuber <felix@obenhuber.de>
A 'adb logcat' wrapper

USAGE:
    rogcat [FLAGS] [OPTIONS] [COMMAND] [SUBCOMMAND]

FLAGS:
    -c, --clear                  Clear (flush) the entire log and exit
    -g, --get-ringbuffer-size    Get the size of the log's ring buffer and exit
    -h, --help                   Prints help information
    -S, --output-statistics      Output statistics
    -r, --restart                Restart command on exit
    -s, --silent                 Do not print on stdout
    -V, --version                Prints version information

OPTIONS:
    -f, --format <format>                        Write format to output files [values: raw, csv]
    -i, --input <INPUT>                          Read from file instead of command. Pass "stdin" to capture stdin
    -l, --level <level>                          Minimum level [values: trace, debug, info, warn, error, fatal, assert, T, D, I, W, E, F, A]
    -m, --msg <MSG>...                           Message filters in RE2
    -o, --output <OUTPUT>                        Write to file and stdout
    -n, --records-per-file <records-per-file>    Write n records per file. Use k, M, G suffixes or a number e.g 9k for 9000
    -t, --tag <TAG>...                           Tag filters in RE2

ARGS:
    <COMMAND>    Optional command to run and capture stdout. Pass "stdin" to capture stdin

SUBCOMMANDS:
    completions    Generates completion scripts for your shell
    help           Prints this message or the help of the given subcommand(s)
```
## Bugs

There are plenty. Report them on GitHub, or - even better - open a pull request.

## Licensing

Rogcat is open source software; see ``COPYING`` for details.
