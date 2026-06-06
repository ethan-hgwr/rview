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

pub(crate) use validate_range;
