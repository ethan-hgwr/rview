const PALETTE: [char; 5] = [' ', '\u{2591}', '\u{2592}', '\u{2593}', '\u{2588}'];

pub(crate) fn map_brightness_to_char(b: f32) -> char {
    let index = (b.clamp(0.0, 1.0) * (PALETTE.len() - 1) as f32).round() as usize;
    PALETTE[index]
}

pub(crate) fn map_brightness_to_rgb(b: f32) -> u32 {
    let light = 0.18 + b.clamp(0.0, 1.0) * 0.82;
    let red = (54.0 * light) as u32;
    let green = (158.0 * light) as u32;
    let blue = (235.0 * light) as u32;
    (red << 16) | (green << 8) | blue
}

#[cfg(test)]
mod tests {
    use super::map_brightness_to_char;

    #[test]
    fn brightness_uses_a_consistent_shading_ramp() {
        assert_eq!(map_brightness_to_char(0.0), ' ');
        assert_eq!(map_brightness_to_char(0.25), '\u{2591}');
        assert_eq!(map_brightness_to_char(0.5), '\u{2592}');
        assert_eq!(map_brightness_to_char(0.75), '\u{2593}');
        assert_eq!(map_brightness_to_char(1.0), '\u{2588}');
    }

    #[test]
    fn brightness_outside_range_is_clamped() {
        assert_eq!(map_brightness_to_char(-1.0), ' ');
        assert_eq!(map_brightness_to_char(2.0), '\u{2588}');
    }
}
