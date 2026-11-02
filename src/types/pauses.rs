
#[derive(Debug, PartialEq)]
pub enum Pause {
    DoubleWhole,
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

impl Pause {
    pub fn beat_from_pause(pause: &Pause) -> f32 {
        match pause {
            Pause::DoubleWhole => 8.0,
            Pause::Whole => 4.0,
            Pause::Half => 2.0,
            Pause::Quarter => 1.0,
            Pause::Eighth => 0.5,
            Pause::Sixteenth => 0.25,
            Pause::ThirtySecond => 0.125,
            Pause::SixtyFourth => 0.0625,
        }
    }

    pub fn pause_from_beat(beat: f32) -> Option<Pause> {
        match beat {
            8.0 => Some(Pause::DoubleWhole),
            4.0 => Some(Pause::Whole),
            2.0 => Some(Pause::Half),
            1.0 => Some(Pause::Quarter),
            0.25 => Some(Pause::Eighth),
            0.125 => Some(Pause::Sixteenth),
            0.0625 => Some(Pause::ThirtySecond),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beat_from_pause() {
        let test_cases = [
            (Pause::DoubleWhole, 8.0),
            (Pause::Whole, 4.0),
            (Pause::Half, 2.0),
            (Pause::Quarter, 1.0),
            (Pause::Eighth, 0.5),
            (Pause::Sixteenth, 0.25),
            (Pause::ThirtySecond, 0.125),
            (Pause::SixtyFourth, 0.0625),
        ];

        for (pause, expected) in test_cases {
            assert_eq!(
                Pause::beat_from_pause(&pause),
                expected,
                "Failed for {:?}",
                pause
            );
        }
    }

    #[test]
    fn pause_from_beat() {
        let test_cases = [
            (8.0, Some(Pause::DoubleWhole)),
            (4.0, Some(Pause::Whole)),
            (2.0, Some(Pause::Half)),
            (1.0, Some(Pause::Quarter)),
            (0.5, Some(Pause::Eighth)),
            (0.25, Some(Pause::Sixteenth)),
            (0.125, Some(Pause::ThirtySecond)),
            (0.0625, Some(Pause::SixtyFourth)),
        ];

        for (beat, expected) in test_cases {
            assert_eq!(
                Pause::pause_from_beat(beat),
                expected,
                "Failed for beat {}",
                beat
            );
        }
    }

    #[test]
    fn pause_from_invalid_beat() {
        let invalid_beats = [0.0, -1.0, 0.75, 3.0, 16.0, f32::NAN, f32::INFINITY];

        for beat in invalid_beats {
            assert_eq!(
                Pause::pause_from_beat(beat),
                None,
                "Expected None for beat {}",
                beat
            );
        }
    }
}