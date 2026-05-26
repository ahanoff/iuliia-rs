//! ALA-LC transliteration schema.
//!
//! See: <https://dangry.ru/iuliia/ala-lc/>

use crate::Schema;

/// ALA-LC transliteration schema..
pub struct AlaLcAlt;

impl Schema for AlaLcAlt {
    const NAME: &'static str = "ala_lc_alt";

    fn mapping(c: char) -> Option<&'static str> {
        match c {
            'а' => Some("a"),
            'б' => Some("b"),
            'в' => Some("v"),
            'г' => Some("g"),
            'д' => Some("d"),
            'е' => Some("e"),
            'ж' => Some("zh"),
            'з' => Some("z"),
            'и' => Some("i"),
            'й' => Some("i"),
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
            'х' => Some("kh"),
            'ц' => Some("ts"),
            'ч' => Some("ch"),
            'ш' => Some("sh"),
            'щ' => Some("shch"),
            'ъ' => Some("\""),
            'ы' => Some("y"),
            'ь' => Some("'"),
            'э' => Some("e"),
            'ю' => Some("iu"),
            'я' => Some("ia"),
            'ё' => Some("e"),
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
            AlaLcAlt::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Iuliia, s\"esh' eshche etikh miagkikh frantsuzskikh bulok iz Ioshkar-Oly, da vypei altaiskogo chaiu"
        );
    }
}
