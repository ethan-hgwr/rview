use crate::{
    camera::CameraState,
    motion::{
        animation::Animation,
        turntable::{Turntable, TurntableConfig},
    },
};

/// Number of seconds to run the bechmark
const TIMEOUT: f32 = 5.0;

pub(crate) struct Benchmark<A>
where
    A: Animation,
{
    animation: A,
    timeout: f32,
}

impl Benchmark<Turntable> {
    pub(crate) fn new() -> Self {
        let config = TurntableConfig::new(3.0, 20.0, 2.0);

        Self {
            animation: Turntable::new(config),
            timeout: TIMEOUT,
        }
    }

    pub(crate) fn is_timedout(&self, elapsed: f32) -> bool {
        self.timeout <= elapsed
    }
}

impl<A> Animation for Benchmark<A>
where
    A: Animation,
{
    fn update(&self, prev: &mut CameraState, dt: f32) {
        self.animation.update(prev, dt);
    }
}
