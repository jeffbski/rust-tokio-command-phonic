use anyhow::{Result, bail};
use futures::StreamExt;
use phonic::{DefaultOutputDevice, FilePlaybackOptions, OutputDevice, Player};
use std::sync::Once;
use tokio::process::Command;

static WINNER: Once = Once::new();

async fn run_cmd(arg: &str) -> Result<String> {
    let out = Command::new("sh").arg("-c").arg(arg).output().await?;
    if !out.status.success() {
        bail!(
            "exit {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Open the default audio device (cpal or sokol, depending on the enabled output feature)
    let device = DefaultOutputDevice::open()?;
    // Create a player and transfer ownership of the audio output to the player.
    let mut player = Player::new(device.sink(), None);

    // Play back a file with the default playback options.
    player.play_file(
    "/Users/jeff/Downloads/drag-race-549905__sherryhanmore__pro_vid_20201221_150557_00_015_2.mp3",
        FilePlaybackOptions::default())?;

    let cmds = vec!["sleep 5; echo a", "sleep 9; echo b", "sleep 4; echo c"];
    let cmd_len = cmds.len();
    let results: Vec<Result<String>> =
        futures::stream::iter(cmds)
            .then(|cmd| async move { run_cmd(cmd) }) // produce futures
            .buffer_unordered(cmd_len) // run all cmds at once
            .then(|result| async move {
                println!("then {result:?}");
                result
            })
            .inspect(|x| {
                println!("{x:?}");
                WINNER.call_once(|| {
                    println!("Winner!!");
                    // Stop all playing files: this will quickly fade-out all playing files to avoid clicks.
                    player.stop_all_sources().unwrap();

                    // Play back a file with the default playback options.
                    player.play_file(
                    "/Users/jeff/Downloads/456966__funwithsound__success-fanfare-trumpets.mp3",
                    FilePlaybackOptions::default(),
                ).unwrap();
                });
            })
            .collect()
            .await;

    println!("{:?}", results);
    Ok(())
}
