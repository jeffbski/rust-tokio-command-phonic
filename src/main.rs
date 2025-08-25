use anyhow::{Result, bail};
use clap::Parser;
use crossbeam_channel::bounded;
use futures::StreamExt;
use phonic::{DefaultOutputDevice, FilePlaybackOptions, OutputDevice, PlaybackStatusEvent, Player};
use std::sync::Arc;
use tokio::{
    process::Command,
    sync::{Mutex, OnceCell},
    task::{self, JoinHandle},
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
    start_sound: Option<std::path::PathBuf>,

    /// Path to the sound file to play when the first command finishes.
    #[arg(short, long)]
    finish_sound: Option<std::path::PathBuf>,
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

    let mut combined_output = out.stdout;
    combined_output.extend(out.stderr);
    Ok(String::from_utf8_lossy(&combined_output).to_string())
}

struct PlayerExt<T> {
    player: Arc<Mutex<Player>>,
    task_join_handle: JoinHandle<T>,
}

fn create_player(audio_to_play_count: i32) -> Result<PlayerExt<usize>> {
    // Create a channel to receive events from the player.
    let (playback_status_sender, playback_status_receiver) = bounded(32);
    let status_join_handle = task::spawn_blocking(move || {
        playback_status_receiver.iter()
            // debug output of the status of the player
            .inspect(|event| {
                    match event {
                        PlaybackStatusEvent::Position {
                            id,
                            path,
                            context,
                            position,
                        } => {
                            println!(
                                "event Position: id:{id}, path:{path}, context:{context:?}, position:{position:?}"
                            );
                        }
                        PlaybackStatusEvent::Stopped {
                            id,
                            path,
                            context,
                            exhausted,
                        } => {
                            println!(
                                "event Stopped: id:{id}, path:{path}, context:{context:?}, exhausted: {exhausted}"
                            );
                        }
                    }
                })
                // filter down to only the Stopped events
                .filter(|event| {
                    match event {
                        PlaybackStatusEvent::Stopped { .. } => true,
                        _ => false
                    }
                })
                // we can exit once we have received the expected stopped events
                .take(audio_to_play_count as usize)
                .count() // we don't really need the count, just using to drive iterator to end
    });

    // Open the default audio device.
    let device = DefaultOutputDevice::open()?;

    // Create a player and wrap it in an Arc<Mutex<>> for safe sharing across threads.
    let player = Arc::new(Mutex::new(Player::new(
        device.sink(),
        Some(playback_status_sender),
    )));
    Ok(PlayerExt {
        player,
        task_join_handle: status_join_handle,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arc::new(Args::parse());
    let mut audio_to_play_count = 0;
    if args.start_sound.is_some() {
        audio_to_play_count += 1;
    }
    if args.finish_sound.is_some() {
        audio_to_play_count += 1;
    }

    let PlayerExt {
        player,
        task_join_handle,
    } = create_player(audio_to_play_count)?;

    // Play the start sound.
    if let Some(start_sound) = &args.start_sound {
        player
            .lock()
            .await
            .play_file(start_sound, FilePlaybackOptions::default())?;
    }

    let commands_str = std::fs::read_to_string(&args.command_file)?;
    let cmds: Vec<String> = commands_str
        .lines()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    let cmd_len = cmds.len();

    let player_clone = Arc::clone(&player);
    let args_clone = Arc::clone(&args);

    let count = futures::stream::iter(cmds)
        .then(|cmd| async move { run_cmd(cmd) }) // Produce futures
        .buffer_unordered(cmd_len) // Run all cmds at once
        .then(|result| {
            // Clone Arcs for use in the async block.
            let player = Arc::clone(&player_clone);
            let args = Arc::clone(&args_clone);

            async move {
                if let Ok(output) = &result {
                    // The first command to finish will initialize the OnceCell.
                    WINNER
                        .get_or_init(|| async {
                            println!("Winner!!");
                            // Stop the startup sound.
                            player
                                .lock()
                                .await
                                .stop_all_sources()
                                .expect("Failed to stop audio sources");

                            // Play the finish sound.
                            if let Some(finish_sound) = &args.finish_sound {
                                player
                                    .lock()
                                    .await
                                    .play_file(finish_sound, FilePlaybackOptions::default())
                                    .expect("Failed to play finish sound");
                            }
                            ()
                        })
                        .await;

                    println!("{output}");
                }

                result
            }
        })
        .count()
        .await;

    println!("All commands finished: {count}");

    task_join_handle.await?; // join to wait if audio has not yet finished playing
    Ok(())
}
