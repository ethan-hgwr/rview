use crate::camera::CameraState;
use clap::Subcommand;

pub(crate) trait Animation {
    fn update(&self, prev: &mut CameraState, dt: f32);
}

#[derive(Subcommand, Clone)]
pub(crate) enum AnimationCommand {
    #[command(allow_negative_numbers = true)]
    Turntable {
        /// The speed at which the model turns. (set to negative to rotate the other way)
        #[arg(long, default_value_t = 5.0)]
        speed: f32,
    },
}
