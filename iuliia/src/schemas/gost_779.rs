//! GOST 7.79-2000 (aka ISO 9:1995) transliteration schema
//!
//! See: <https://dangry.ru/iuliia/gost-779/>

use crate::Schema;

/// GOST 7.79-2000 (aka ISO 9:1995) transliteration schema.
pub struct Gost779;

impl Schema for Gost779 {
    const NAME: &'static str = "gost_779";
    const ALIASES: &'static [&'static str] = &[
        "iso_9_1995",
    ];

    fn mapping(c: char) -> Option<&'static str> {
        match c {
            'а' => Some("a"),
            'б' => Some("b"),
            'в' => Some("v"),
            'г' => Some("g"),
            'д' => Some("d"),
            'е' => Some("e"),
            'ж' => Some("z\u{30c}"),
            'з' => Some("z"),
            'и' => Some("i"),
            'й' => Some("j"),
            'к' => Some("k"),
            'л' => Some("l"),
            'м' => Some("m"),
            'н' => Some("n"),
            'о' => Some("o"),
            'п' => Some("p"),
            'р' => Some("r"),
            'с' => Some("s"),
            'т' => Some("t"),
            'у' => Some("u"),
            'ф' => Some("f"),
            'х' => Some("h"),
            'ц' => Some("c"),
            'ч' => Some("č"),
            'ш' => Some("š"),
            'щ' => Some("ŝ"),
            'ъ' => Some("ʺ"),
            'ы' => Some("y"),
            'ь' => Some("ʹ"),
            'э' => Some("è"),
            'ю' => Some("û"),
            'я' => Some("â"),
            'ё' => Some("ё"),
            _ => None,
        }
    }

    fn prev_mapping(_prev: Option<char>, _curr: char) -> Option<&'static str> {
        None
    }

    fn next_mapping(_curr: char, _next: char) -> Option<&'static str> {
        None
    }

    fn ending_mapping(_ending: [char; 2]) -> Option<&'static str> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        assert_eq!(
            Gost779::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Ûliâ, sʺešʹ eŝё ètih mâgkih francuzskih bulok iz Joškar-Oly, da vypej altajskogo čaû"
        );
    }
}
