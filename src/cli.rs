use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(about = "Render .obj models as ASCII art in your terminal, with real-time camera rotation and zoom. Built with Rust 🦀", long_about = None)]
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
}
