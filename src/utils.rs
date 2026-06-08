pub(crate) fn validate(name: &str, val: f32, min: f32, max: f32) -> anyhow::Result<()> {
    if (min..=max).contains(&val) {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "{name} must be between {min} and {max}, got {val}"
        ))
    }
}

macro_rules! validate_range {
    ($val:expr, $min:expr, $max:expr) => {
        $crate::utils::validate(stringify!($val), $val, $min, $max)?
    };
}

pub(crate) use validate_range;
