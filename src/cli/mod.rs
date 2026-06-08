use std::path::PathBuf;

use anyhow::{Ok, Result};
use clap::Parser;

use crate::{
    benchmark::Benchmark,
    camera::CameraState,
    cli::command::{CameraArgs, Commands},
    constants::*,
    mode::Mode,
    motion::turntable::Turntable,
    utils::validate_range,
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

    /// Run a benchmark with fixed parameters
    #[arg(long, default_value_t = false)]
    pub(crate) bench: bool,

    /// Commands
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

fn validate_camera_args(args: &CameraArgs) -> Result<()> {
    // Since `validate_range` stringifies the args we remove the prefix "args.x"
    let pitch = args.pitch;
    let radius = args.radius;
    validate_range!(pitch, CameraState::MIN_PITCH, CameraState::MAX_PITCH);
    validate_range!(radius, MIN_CAM_RADIUS, MAX_CAM_RADIUS);

    Ok(())
}

pub(crate) fn resolve_cli(cli: &Cli) -> Result<(Mode, CameraState)> {
    // Since `validate_range` stringifies the args we remove the prefix "cli.x"
    let fov = cli.fov;
    let near = cli.near;
    let far = cli.far;

    if cli.bench {
        return Ok((Mode::Benchmark(Benchmark::new()), CameraState::default()));
    }

    validate_range!(fov, Cli::MIN_FOV, Cli::MAX_FOV);
    validate_range!(near, Cli::MIN_NEAR, far);
    validate_range!(far, near, Cli::MAX_FAR);

    if let Some(cmd) = &cli.command {
        return match cmd {
            Commands::Turntable { speed, camera } => {
                validate_camera_args(camera)?;

                let state = CameraState::new(
                    camera.yaw.to_radians(),
                    camera.pitch.to_radians(),
                    camera.radius,
                );
                Ok((Mode::Animation(Box::new(Turntable::new(*speed))), state))
            }
            Commands::Interactive { camera } => {
                validate_camera_args(camera)?;

                let state = CameraState::new(
                    camera.yaw.to_radians(),
                    camera.pitch.to_radians(),
                    camera.radius,
                );
                Ok((Mode::Interactive, state))
            }
        };
    }

    Ok((Mode::Interactive, CameraState::default()))
}
