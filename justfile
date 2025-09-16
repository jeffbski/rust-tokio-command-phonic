# build release version
build:
    cargo build --release

# build debug version and run tests
test:
    cargo test

# run with default commands and sounds from commands.txt
run:
    ./target/release/rust-tokio-command-phonic -c commands.txt -s /Users/jeff/Downloads/drag-race-549905__sherryhanmore__pro_vid_20201221_150557_00_015_2.mp3 -f /Users/jeff/Downloads/456966__funwithsound__success-fanfare-trumpets.mp3

# run with no sound from commands.txt
run-silent:
    ./target/release/rust-tokio-command-phonic -c commands.txt

# run with real commands and sounds from real-commands.txt
real:
    ./target/release/rust-tokio-command-phonic -c real-commands.txt -s /Users/jeff/Downloads/race-car-362035.mp3 -f /Users/jeff/Downloads/456966__funwithsound__success-fanfare-trumpets.mp3 | rg "Output|Count|real|resident"

# run with real commands but no sound from real-commands.txt
real-silent:
    ./target/release/rust-tokio-command-phonic -c real-commands.txt | rg "Output|Count|real|resident"

# run with real commands and sounds and full phonic debug output from real-commands.txt
real-with-debug:
    ./target/release/rust-tokio-command-phonic -c real-commands.txt -s /Users/jeff/Downloads/race-car-362035.mp3 -f /Users/jeff/Downloads/456966__funwithsound__success-fanfare-trumpets.mp3

