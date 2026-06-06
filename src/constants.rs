use crate::{benchmark::Benchmark, camera::CameraState, cli::Cli, motion::turntable::Turntable};

/* --- Rendering --- */

/// Target frames per second.
pub(crate) const TARGET_FPS: u32 = 165;

/// Duration of a single frame in seconds.
pub(crate) const REFRESH_RATE: f32 = 1.0 / TARGET_FPS as f32;

/// Character used to clear the framebuffer.
pub(crate) const BACKGROUND: char = ' ';

/* --- Input --- */

/// Key that exits the program.
pub(crate) const EXIT_KEY: char = 'q';

/// Mouse sensitivity for yaw rotation.
pub(crate) const YAW_SENSITIVITY: f32 = 0.05;

/// Mouse sensitivity for pitch rotation. Negative to invert vertical axis.
pub(crate) const PITCH_SENSITIVITY: f32 = -YAW_SENSITIVITY;

/* --- Camera --- */

/// Minimum distance from the camera to the origin.
pub(crate) const MIN_CAM_RADIUS: f32 = 1.0;

/// Maximum distance from the camera to the origin.
pub(crate) const MAX_CAM_RADIUS: f32 = 10.0;

/// Distance increment per scroll event.
pub(crate) const CAM_RADIUS_STEP: f32 = 0.1;

impl CameraState {
    /// Default yaw angle in radians.
    pub(crate) const DEFAULT_YAW: f32 = 180.0_f32.to_radians();

    /// Default pitch angle in radians.
    pub(crate) const DEFAULT_PITCH: f32 = 0.0_f32.to_radians();

    /// Default distance from the origin.
    pub(crate) const DEFAULT_RADIUS: f32 = 2.0;
}

/* --- Projection --- */

impl Cli {
    /// Default field of view in degrees.
    pub(crate) const DEFAULT_FOV: f32 = 40.0;

    /// Default near clipping plane distance.
    pub(crate) const DEFAULT_NEAR: f32 = 0.01;

    /// Default far clipping plane distance.
    pub(crate) const DEFAULT_FAR: f32 = 10.0;

    /// Minimum allowed field of view in degrees.
    pub(crate) const MIN_FOV: f32 = 0.0;

    /// Maximum allowed field of view in degrees.
    pub(crate) const MAX_FOV: f32 = 100.0;

    /// Minimum allowed near clipping plane distance.
    pub(crate) const MIN_NEAR: f32 = 0.0;

    /// Maximum allowed far clipping plane distance.
    pub(crate) const MAX_FAR: f32 = 100.0;
}

/* --- Animation --- */

impl Turntable {
    /// Default rotation speed in radians per second.
    pub(crate) const DEFAULT_SPEED: f32 = 5.0;
}

/* --- Benchmark --- */

impl Benchmark<Turntable> {
    /// Duration of the benchmark in seconds.
    pub(crate) const TIMEOUT: f32 = 5.0;

    /// Turntable rotation speed used during the benchmark in radians per second.
    pub(crate) const SPEED: f32 = 3.0;
}
