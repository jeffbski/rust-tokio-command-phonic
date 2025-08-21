use anyhow::{Result, bail};
use clap::Parser;
use futures::StreamExt;
use phonic::{DefaultOutputDevice, FilePlaybackOptions, OutputDevice, Player};
use std::sync::Arc;
use tokio::{
    process::Command,
    sync::{Mutex, OnceCell},
};

/// A simple program that runs commands in parallel, plays a sound on start,
/// and plays another sound when the first command finishes.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the file containing commands to run, one per line.
    #[arg(short, long)]
    command_file: std::path::PathBuf,

    /// Path to the sound file to play on start.
    #[arg(short, long)]
    start_sound: std::path::PathBuf,

    /// Path to the sound file to play when the first command finishes.
    #[arg(short, long)]
    finish_sound: std::path::PathBuf,
}

static WINNER: OnceCell<()> = OnceCell::const_new();

async fn run_cmd(arg: String) -> Result<String> {
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
    let args = Arc::new(Args::parse());

    // Open the default audio device.
    let device = DefaultOutputDevice::open()?;
    // Create a player and wrap it in an Arc<Mutex<>> for safe sharing across threads.
    let player = Arc::new(Mutex::new(Player::new(device.sink(), None)));

    // Play the start sound.
    player
        .lock()
        .await
        .play_file(&args.start_sound, FilePlaybackOptions::default())?;

    let commands_str = std::fs::read_to_string(&args.command_file)?;
    let cmds: Vec<String> = commands_str
        .lines()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    let cmd_len = cmds.len();

    let player_clone = Arc::clone(&player);
    let args_clone = Arc::clone(&args);

    let results: Vec<Result<String>> = futures::stream::iter(cmds)
        .then(|cmd| async move { run_cmd(cmd) }) // Produce futures
        .buffer_unordered(cmd_len) // Run all cmds at once
        .then(|result| {
            // Clone Arcs for use in the async block.
            let player = Arc::clone(&player_clone);
            let args = Arc::clone(&args_clone);
            async move {
                println!("Finished: {result:?}");

                // The first command to finish will initialize the OnceCell.
                WINNER
                    .get_or_init(|| async {
                        println!("Winner!!");
                        let mut p = player.lock().await;
                        // Stop the startup sound.
                        p.stop_all_sources().expect("Failed to stop sources");

                        // Play the finish sound.
                        p.play_file(&args.finish_sound, FilePlaybackOptions::default())
                            .expect("Failed to play finish sound");
                    })
                    .await;

                result
            }
        })
        .collect()
        .await;

    println!("All commands finished: {:?}", results);

    // Give the final sound a moment to play before exiting.
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    Ok(())
}
