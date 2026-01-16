mod cargo;
mod tracing;

use std::{path::PathBuf, process::Stdio, time::Duration};

use clap::{Parser, Subcommand, ValueHint};
use color_eyre::eyre::Result;
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{DebouncedEvent, Debouncer, new_debouncer};
use tokio::{
    process::{Child, Command},
    sync::mpsc::{Receiver, channel},
    task,
};
use tracing_core::LevelFilter;

use crate::{cargo::Cargo, tracing::OutputOptions};

#[derive(Debug, Parser)]
struct Xtask {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
    #[clap(flatten)]
    pub options: Options,
    #[clap(flatten)]
    pub output: tracing::OutputOptions,
}

#[derive(Debug, Parser, Clone)]
struct Options {
    /// Build kernel & loader in release mode, with optimizations enabled
    #[clap(long, global = true)]
    pub release: bool,

    /// Overrides the path to the `cargo` executable.
    ///
    /// By default, this is read from the `CARGO` environment variable.
    #[clap(
        long = "cargo",
        env = "CARGO",
        default_value = "cargo",
        value_hint = ValueHint::ExecutablePath,
        global = true
    )]
    pub cargo_path: PathBuf,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    Watch,
}

#[tokio::main]
async fn main() -> Result<()> {
    let xtask = Xtask::parse();

    xtask.output.init_tracing_subscriber()?;

    match xtask.subcommand {
        SubCommand::Watch => {
            task::spawn(async {
                let mut trunk = KillOnDrop(
                    Command::new("trunk")
                        .args([
                            "serve",
                            "--config",
                            "frontend",
                            "--watch",
                            "../transformers",
                            "--open",
                            "--proxy-backend",
                            "http://localhost:3000/api",
                        ])
                        .spawn()
                        .unwrap(),
                );

                trunk.0.wait().await.unwrap();
            });

            watch(xtask.options.clone(), xtask.output.clone()).await?
        }
    }

    Ok(())
}

async fn watch(opts: Options, output: OutputOptions) -> crate::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watcher()
        .watch("./transformers".as_ref(), RecursiveMode::Recursive)?;

    let mut task = task::spawn(run_backend(opts.clone(), output.clone()));

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => {
                ::tracing::info!(
                    "Changed `{}` Rebuilding...",
                    event[0].path.to_str().unwrap_or("<unknown>")
                );

                let abort_handle = task.abort_handle();
                abort_handle.abort();
                task.await.unwrap_err();

                task = task::spawn(run_backend(opts.clone(), output.clone()));
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

async fn run_backend(opts: Options, output: OutputOptions) -> crate::Result<()> {
    let mut cmd = Cargo::run("backend", &opts, &output)?.into_cmd();

    if output.log.default_level().unwrap_or(LevelFilter::INFO) < LevelFilter::DEBUG {
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
    }

    let mut child = KillOnDrop(cmd.spawn()?);
    child.0.wait().await?;
    Ok(())
}

fn async_watcher() -> notify::Result<(
    Debouncer<RecommendedWatcher>,
    Receiver<notify::Result<Vec<DebouncedEvent>>>,
)> {
    let (tx, rx) = channel(1);

    // no specific tickrate, max debounce time 1 seconds
    let debouncer = new_debouncer(Duration::from_secs(1), move |res| {
        tx.blocking_send(res).unwrap()
    })
    .unwrap();

    Ok((debouncer, rx))
}

pub struct KillOnDrop(pub Child);

impl Drop for KillOnDrop {
    fn drop(&mut self) {
        self.0.start_kill().unwrap()
    }
}
