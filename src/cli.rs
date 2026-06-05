use std::path::PathBuf;

use anyhow::{Ok, Result};
use clap::Parser;

use crate::{
    benchmark::Benchmark,
    mode::Mode,
    motion::{
        animation::AnimationCommand,
        turntable::{Turntable, TurntableConfig},
    },
};

macro_rules! validate_range {
    ($val:expr, $min:expr, $max:expr) => {
        if !($val >= $min && $val <= $max) {
            return Err(anyhow::anyhow!(
                "{} must be between {} and {}, got {}",
                stringify!($val),
                $min,
                $max,
                $val
            ));
        }
    };
}

#[derive(Parser)]
#[command(about = "Render .obj models as ASCII art in your terminal, with real-time camera rotation and zoom. Built with Rust 🦀", long_about = None, allow_negative_numbers = true)]
pub(crate) struct Cli {
    /// File path
    pub(crate) file_path: PathBuf,

    /// Initial yaw of the camera in degrees
    #[arg(short, long, default_value_t = 180.0)]
    pub(crate) yaw: f32,

    /// Initial pitch of the camera in degrees
    #[arg(short, long, default_value_t = 0.0)]
    pub(crate) pitch: f32,

    /// Initial distance of the camera from the origin
    #[arg(short, long, default_value_t = 2.0)]
    pub(crate) distance: f32,

    /// Fov in degrees
    #[arg(long, default_value_t = 40.0)]
    pub(crate) fov: f32,

    /// Near clipping plane
    #[arg(long, default_value_t = 0.01)]
    pub(crate) near: f32,

    /// Far clipping plane
    #[arg(long, default_value_t = 10.0)]
    pub(crate) far: f32,

    /// Run a fixed camera benchmark and exit
    #[arg(long, default_value_t = false)]
    pub(crate) bench: bool,

    /// Animation type
    #[command(subcommand)]
    pub(crate) animation: Option<AnimationCommand>,
}

pub(crate) fn build_mode(cli: &Cli) -> Result<Mode> {
    validate_range!(cli.yaw, 0.0, 360.0);
    validate_range!(cli.pitch, -180.0, 180.0);
    validate_range!(cli.distance, 1.0, 10.0);
    validate_range!(cli.fov, 0.0, 100.0);
    validate_range!(cli.near, 0.0, cli.far);
    validate_range!(cli.far, cli.near, 100.0);

    if cli.bench {
        return Ok(Mode::Benchmark(Benchmark::new()));
    }

    if let Some(animation) = &cli.animation {
        return match animation {
            AnimationCommand::Turntable { speed } => {
                let config = TurntableConfig::new(*speed, cli.pitch, cli.distance);
                Ok(Mode::Animation(Box::new(Turntable::new(config))))
            }
        };
    }

    Ok(Mode::Interactive)
}
