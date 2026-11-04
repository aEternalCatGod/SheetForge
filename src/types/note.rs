
#[derive(Debug, PartialEq)]
pub enum Note {
    DoubleWhole,
    DottedDoubleWhole,
    Whole,
    DottedWhole,
    Half,
    DottedHalf,
    Quarter,
    DottedQuarter,
    Eighth,
    DottedEighth,
    Sixteenth,
    DottedSixteenth,
    ThirtySecond,
    DottedThirtySecond,
    SixtyFourth,
    DottedSixtyFourth,
}

impl Note {
    pub fn beat_from_note(note: &Note) -> f32 {
        match note {
            Note::DoubleWhole => 8.0,
            Note::DottedDoubleWhole => 12.0,
            Note::Whole => 4.0,
            Note::DottedWhole => 6.0,
            Note::Half => 2.0,
            Note::DottedHalf => 3.0,
            Note::Quarter => 1.0,
            Note::DottedQuarter => 1.5,
            Note::Eighth => 0.5,
            Note::DottedEighth => 0.75,
            Note::Sixteenth => 0.25,
            Note::DottedSixteenth => 0.375,
            Note::ThirtySecond => 0.125,
            Note::DottedThirtySecond => 0.1875,
            Note::SixtyFourth => 0.0625,
            Note::DottedSixtyFourth => 0.09375,
        }
    }

    pub fn note_from_beat(beat: f32) -> Option<Note> {
        match beat {
            8.0 => Some(Note::DoubleWhole),
            12.0 => Some(Note::DottedDoubleWhole),
            4.0 => Some(Note::Whole),
            6.0 => Some(Note::DottedWhole),
            2.0 => Some(Note::Half),
            3.0 => Some(Note::DottedHalf),
            1.0 => Some(Note::Quarter),
            1.5 => Some(Note::DottedQuarter),
            0.5 => Some(Note::Eighth),
            0.75 => Some(Note::DottedEighth),
            0.25 => Some(Note::Sixteenth),
            0.375 => Some(Note::DottedSixteenth),
            0.125 => Some(Note::ThirtySecond),
            0.1875 => Some(Note::DottedThirtySecond),
            0.0625 => Some(Note::SixtyFourth),
            0.09375 => Some(Note::DottedSixtyFourth),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beat_from_note() {
        let test_cases = [
            (Note::DoubleWhole, 8.0),
            (Note::DottedDoubleWhole, 12.0),
            (Note::Whole, 4.0),
            (Note::DottedWhole, 6.0),
            (Note::Half, 2.0),
            (Note::DottedHalf, 3.0),
            (Note::Quarter, 1.0),
            (Note::DottedQuarter, 1.5),
            (Note::Eighth, 0.5),
            (Note::DottedEighth, 0.75),
            (Note::Sixteenth, 0.25),
            (Note::DottedSixteenth, 0.375),
            (Note::ThirtySecond, 0.125),
            (Note::DottedThirtySecond, 0.1875),
            (Note::SixtyFourth, 0.0625),
            (Note::DottedSixtyFourth, 0.09375),
        ];

        for (note, expected) in test_cases {
            assert_eq!(
                Note::beat_from_note(&note),
                expected,
                "Failed for {:?}",
                note
            );
        }
    }

    #[test]
    fn note_from_beat() {
        let test_cases = [
            (8.0, Some(Note::DoubleWhole)),
            (12.0, Some(Note::DoubleWhole)),
            (4.0, Some(Note::Whole)),
            (6.0, Some(Note::DoubleWhole)),
            (2.0, Some(Note::Half)),
            (3.0, Some(Note::DottedHalf)),
            (1.0, Some(Note::Quarter)),
            (1.5, Some(Note::DottedQuarter)),
            (0.5, Some(Note::Eighth)),
            (0.75, Some(Note::DottedEighth)),
            (0.25, Some(Note::Sixteenth)),
            (0.375, Some(Note::DottedSixteenth)),
            (0.125, Some(Note::ThirtySecond)),
            (0.1875, Some(Note::DottedThirtySecond)),
            (0.0625, Some(Note::SixtyFourth)),
            (0.09375, Some(Note::DottedSixtyFourth)),
        ];

        for (beat, expected) in test_cases {
            assert_eq!(
                Note::note_from_beat(beat),
                expected,
                "Failed for beat {}",
                beat
            );
        }
    }

    #[test]
    fn note_from_invalid_beat() {
        let invalid_beats = [0.0, -1.0, 0.75, 3.0, 16.0, f32::NAN, f32::INFINITY];

        for beat in invalid_beats {
            assert_eq!(
                Note::note_from_beat(beat),
                None,
                "Expected None for beat {}",
                beat
            );
        }
    }
}