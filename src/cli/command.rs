use clap::Subcommand;

use crate::{CameraState, motion::turntable::Turntable};

#[derive(clap::Args, Clone)]
pub(crate) struct CameraArgs {
    /// Initial yaw of the camera in degrees (might get overriden depending on the choosen mode)
    #[arg(short, long, default_value_t = CameraState::DEFAULT_YAW)]
    pub(crate) yaw: f32,

    /// Initial pitch of the camera in degrees (might get overriden depending on the choosen mode)
    #[arg(short, long, default_value_t = CameraState::DEFAULT_PITCH)]
    pub(crate) pitch: f32,

    /// Initial radius of the camera from the origin (might get overriden depending on the choosen mode)
    #[arg(short, long, default_value_t = CameraState::DEFAULT_RADIUS)]
    pub(crate) radius: f32,
}

#[derive(Subcommand, Clone)]
pub(crate) enum Commands {
    #[command(allow_negative_numbers = true)]
    Turntable {
        /// The speed at which the model turns. (set to negative to rotate the other way)
        #[arg(long, default_value_t = Turntable::DEFAULT_SPEED)]
        speed: f32,

        #[command(flatten)]
        camera: CameraArgs,
    },

    #[command(allow_negative_numbers = true)]
    Interactive {
        #[command(flatten)]
        camera: CameraArgs,
    },
}
