use crate::generator::ChordQuality;

pub fn note_name(pitch: u8) -> String {
    let names = [
        "C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
    ];
    format!("{}{}", names[(pitch % 12) as usize], pitch / 12 - 1)
}

pub(crate) fn pitch_class_name(pitch_class: u8) -> &'static str {
    let names = [
        "C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
    ];
    names[(pitch_class % 12) as usize]
}

pub(crate) fn roman_degree(degree: usize, quality: ChordQuality, tension: u8) -> String {
    let base = match degree % 7 {
        0 => "I",
        1 => "II",
        2 => "III",
        3 => "IV",
        4 => "V",
        5 => "VI",
        _ => "VII",
    };
    let cased = match quality {
        ChordQuality::Minor
        | ChordQuality::Diminished
        | ChordQuality::MinorDyad
        | ChordQuality::Minor7 => base.to_lowercase(),
        _ => base.to_string(),
    };
    if tension > 65 {
        format!("{cased}+")
    } else {
        cased
    }
}
