pub enum Note {
    DoubleWhole,
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

impl Note {
    pub fn beat_from_note(note: &Note) -> f32 {
        match note {
            Note::DoubleWhole => 8.0,
            Note::Whole => 4.0,
            Note::Half => 2.0,
            Note::Quarter => 1.0,
            Note::Eighth => 0.5,
            Note::Sixteenth => 0.25,
            Note::ThirtySecond => 0.125,
            Note::SixtyFourth => 0.0625,
        }
    }

    pub fn note_from_beat(beat: f32) -> Option<Note> {
        match beat {
            8.0 => Some(Note::DoubleWhole),
            4.0 => Some(Note::Whole),
            2.0 => Some(Note::Half),
            1.0 => Some(Note::Quarter),
            0.25 => Some(Note::Eighth),
            0.125 => Some(Note::Sixteenth),
            0.0625 => Some(Note::ThirtySecond),
            _ => None,
        }
    }
}