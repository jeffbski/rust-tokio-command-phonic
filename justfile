
# run with default commands and sounds
run:
    ./target/debug/rust-tokio-command-phonic -c commands.txt -s /Users/jeff/Downloads/drag-race-549905__sherryhanmore__pro_vid_20201221_150557_00_015_2.mp3 -f /Users/jeff/Downloads/456966__funwithsound__success-fanfare-trumpets.mp3


# run with no sound
run-silent:
    ./target/debug/rust-tokio-command-phonic -c commands.txt

# run with real commands and sounds
real:
    ./target/debug/rust-tokio-command-phonic -c real-commands.txt -s /Users/jeff/Downloads/race-car-362035.mp3 -f /Users/jeff/Downloads/456966__funwithsound__success-fanfare-trumpets.mp3 | rg "Output|Count|real|resident"

# run with real commands but no sound
real-silent:
    ./target/debug/rust-tokio-command-phonic -c real-commands.txt | rg "Output|Count|real|resident"

# run with real commands and sounds and full debug output
real-with-debug:
    ./target/debug/rust-tokio-command-phonic -c real-commands.txt -s /Users/jeff/Downloads/race-car-362035.mp3 -f /Users/jeff/Downloads/456966__funwithsound__success-fanfare-trumpets.mp3

