# rust-tokio-command-phonic

A rust project which launches tokio commands concurrently and plays audio on start and the first command to finish

## Usage

A simple program that runs commands in parallel, plays a sound on start, and plays another sound when the first command finishes

Usage: rust-tokio-command-phonic [OPTIONS] --command-file <COMMAND_FILE>

Options:
  -c, --command-file <COMMAND_FILE>  Path to the file containing commands to run, one per line
  -s, --start-sound <START_SOUND>    Path to the sound file to play on start
  -f, --finish-sound <FINISH_SOUND>  Path to the sound file to play when the first command finishes
  -h, --help                         Print help
  -V, --version                      Print version

## Justfile

> just -l

Available recipes:
    build           # build release version
    real            # run with real commands and sounds from real-commands.txt
    real-silent     # run with real commands but no sound from real-commands.txt
    real-with-debug # run with real commands and sounds and full phonic debug output from real-commands.txt
    run             # run with default commands and sounds from commands.txt
    run-silent      # run with no sound from commands.txt
    test            # build debug version and run tests
