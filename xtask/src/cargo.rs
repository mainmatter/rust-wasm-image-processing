use std::path::Path;
use tokio::process::Command;

use tracing_core::LevelFilter;

use crate::{
    Options,
    tracing::{ColorMode, OutputOptions},
};

pub struct Cargo<'a> {
    cmd: &'a str,
    cargo_path: &'a Path,
    verbosity: u8,
    release: bool,
    color_mode: ColorMode,
    krate: &'a str,
}

impl<'a> Cargo<'a> {
    fn new(cmd: &'a str, krate: &'a str, opts: &'a Options, output: &OutputOptions) -> Self {
        let verbosity = output.log.default_level().map_or(0, |lvl| match lvl {
            LevelFilter::TRACE => 2,
            LevelFilter::DEBUG => 1,
            _ => 0,
        });

        Self {
            cmd,
            cargo_path: &opts.cargo_path,
            verbosity,
            release: opts.release,
            color_mode: output.color,
            krate,
        }
    }

    pub fn run(krate: &'a str, opts: &'a Options, output: &OutputOptions) -> crate::Result<Self> {
        let this = Self::new("run", krate, opts, output);

        Ok(this)
    }

    pub fn into_cmd(self) -> Command {
        let mut cmd = Command::new(&self.cargo_path);
        cmd.args([self.cmd, "-p", self.krate]);

        cmd.env("CARGO_TERM_COLOR", self.color_mode.as_str());

        if self.release {
            cmd.arg("--release");
        }

        // pass on the number of `--verbose` flags we received
        if self.verbosity > 0 {
            cmd.arg(format!("-{}", str::repeat("v", self.verbosity as usize)));
        }

        cmd
    }
}
