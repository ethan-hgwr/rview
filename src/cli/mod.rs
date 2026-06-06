use std::path::PathBuf;

use anyhow::{Ok, Result};
use clap::Parser;

use crate::{
    benchmark::Benchmark, camera::CameraState, cli::command::Commands, mode::Mode,
    motion::turntable::Turntable, utils::validate_range,
};

mod command;

#[derive(Parser)]
#[command(about = "Render .obj models as ASCII art in your terminal, with real-time camera rotation and zoom. Built with Rust 🦀", long_about = None, allow_negative_numbers = true)]
pub(crate) struct Cli {
    /// File path
    pub(crate) file_path: PathBuf,

    /// Fov in degrees
    #[arg(long, default_value_t = Cli::DEFAULT_FOV)]
    pub(crate) fov: f32,

    /// Near clipping plane
    #[arg(long, default_value_t = Cli::DEFAULT_NEAR)]
    pub(crate) near: f32,

    /// Far clipping plane
    #[arg(long, default_value_t = Cli::DEFAULT_FAR)]
    pub(crate) far: f32,

    /// Run a fixed camera benchmark and exit
    #[arg(long, default_value_t = false)]
    pub(crate) bench: bool,

    /// Commands
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

pub(crate) fn resolve_cli(cli: &Cli) -> Result<(Mode, CameraState)> {
    validate_range!(cli.fov, Cli::MIN_FOV, Cli::MAX_FOV);
    validate_range!(cli.near, Cli::MIN_NEAR, cli.far);
    validate_range!(cli.far, cli.near, Cli::MAX_FAR);

    if cli.bench {
        return Ok((Mode::Benchmark(Benchmark::new()), CameraState::default()));
    }

    if let Some(cmd) = &cli.command {
        return match cmd {
            Commands::Turntable { speed, camera } => {
                let state = CameraState::new(camera.yaw, camera.pitch, camera.radius);
                Ok((Mode::Animation(Box::new(Turntable::new(*speed))), state))
            }
            Commands::Interactive { camera } => {
                let state = CameraState::new(camera.yaw, camera.pitch, camera.radius);
                Ok((Mode::Interactive, state))
            }
        };
    }

    Ok((Mode::Interactive, CameraState::default()))
}
