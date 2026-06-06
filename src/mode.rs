use crate::{
    benchmark::Benchmark,
    motion::{Animation, turntable::Turntable},
};

pub(crate) enum Mode {
    Interactive,
    Animation(Box<dyn Animation>),
    Benchmark(Benchmark<Turntable>),
}
