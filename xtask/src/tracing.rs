use std::fmt;
use std::io::IsTerminal;

use clap::{ArgGroup, Args};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const ARG_GROUP: &str = "output-opts";

#[derive(Copy, Clone, Debug, Eq, PartialEq, clap::ValueEnum)]
#[repr(u8)]
#[clap(rename_all = "lower")]
pub enum ColorMode {
    /// Determine whether to color output based on whether or not stderr is a
    /// TTY.
    Auto = 0,
    /// Always color output.
    Always = 1,
    /// Never color output.
    Never = 2,
}

#[derive(Debug, Args, Clone)]
#[command(
    next_help_heading = "Output Options",
    group = ArgGroup::new(ARG_GROUP).multiple(true),
)]
pub struct OutputOptions {
    /// Whether to emit colors in output.
    #[clap(
        long,
        env = "CARGO_TERM_COLOR",
        default_value_t = ColorMode::Auto,
        global = true,
        group = ARG_GROUP,
    )]
    pub color: ColorMode,

    /// Configures build logging.
    #[clap(
        short,
        long,
        env = "RUST_LOG",
        default_value = "xtask=info,warn",
        global = true,
        group = ARG_GROUP,
    )]
    pub log: tracing_subscriber::filter::Targets,
}

// === impl ColorMode ===

impl fmt::Display for ColorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ColorMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorMode::Auto => "auto",
            ColorMode::Always => "always",
            ColorMode::Never => "never",
        }
    }

    pub fn should_color(self) -> bool {
        match self {
            ColorMode::Auto => std::io::stderr().is_terminal(),
            ColorMode::Always => true,
            ColorMode::Never => false,
        }
    }
}

// === impl OutputOptions ===

impl OutputOptions {
    pub fn init_tracing_subscriber(&self) -> crate::Result<()> {
        tracing_subscriber::registry()
            .with(tracing_error::ErrorLayer::default())
            .with(self.log.clone())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(self.color.should_color())
                    .with_target(false)
                    .with_level(true)
                    .compact(),
            )
            .try_init()?;

        Ok(())
    }
}
