#[derive(Debug, PartialEq)]
enum Trend {
    Up,
    Down,
    Flat,
}

// get trend by percent, if percent >= 0.5, then return up, if percent <= -0.5, then return down, if percent > -0.5 and < 0.5, then return flat
fn get_trend(percent: f64) -> Trend {
    if percent >= 0.5 {
        Trend::Up
    } else if percent <= -0.5 {
        Trend::Down
    } else {
        Trend::Flat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_get_trend_true() {
        assert_eq!(get_trend(0.5), Trend::Up);
        assert_eq!(get_trend(-0.5), Trend::Down);
        assert_eq!(get_trend(0.0), Trend::Flat);
    }

    #[test]
    #[ignore]
    fn test_get_trend_false() {
        assert_ne!(get_trend(0.5), Trend::Down);
        assert_ne!(get_trend(-0.5), Trend::Up);
    }
}
