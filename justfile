
# run with default commands and sounds
run:
    ./target/debug/rust-tokio-command-phonic -c commands.txt -s /Users/jeff/Downloads/drag-race-549905__sherryhanmore__pro_vid_20201221_150557_00_015_2.mp3 -f /Users/jeff/Downloads/456966__funwithsound__success-fanfare-trumpets.mp3

# run with real commands and sounds
real:
    ./target/debug/rust-tokio-command-phonic -c real-commands.txt -s /Users/jeff/Downloads/drag-race-549905__sherryhanmore__pro_vid_20201221_150557_00_015_2.mp3 -f /Users/jeff/Downloads/456966__funwithsound__success-fanfare-trumpets.mp3 | rg "Output|Count|real|resident"
