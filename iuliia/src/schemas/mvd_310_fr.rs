//! MVD 310-1997 transliteration schema
//!
//! See: <https://dangry.ru/iuliia/mvd-310/>
//!
//! This schema defines the following rule for the French mapping:
//! \> `С` between two vowels → `SS`
//! There is no such rule in other schemas, and MVD-310 itself is deprecated,
//! so I decided to ignore this specific rule for the sake of code simplicity.

use crate::Schema;

/// MVD 310-1997 transliteration schema.
pub struct Mvd310Fr;

impl Schema for Mvd310Fr {
    const NAME: &'static str = "mvd_310_fr";

    fn mapping(c: char) -> Option<&'static str> {
        match c {
            'а' => Some("a"),
            'б' => Some("b"),
            'в' => Some("v"),
            'г' => Some("g"),
            'д' => Some("d"),
            'е' => Some("e"),
            'ж' => Some("j"),
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
            'у' => Some("ou"),
            'ф' => Some("f"),
            'х' => Some("kh"),
            'ц' => Some("ts"),
            'ч' => Some("tch"),
            'ш' => Some("ch"),
            'щ' => Some("chtch"),
            'ъ' => Some(""),
            'ы' => Some("y"),
            'ь' => Some(""),
            'э' => Some("e"),
            'ю' => Some("iou"),
            'я' => Some("ia"),
            'ё' => Some("e"),
            _ => None,
        }
    }

    fn prev_mapping(prev: Option<char>, curr: char) -> Option<&'static str> {
        match (prev, curr) {
            (Some('г'), 'е') => Some("ue"),
            (Some('г'), 'и') => Some("ui"),
            (Some('г'), 'ы') => Some("uy"),
            (Some('к'), 'с') => Some("x"),
            (Some('ь'), 'е') => Some("ie"),
            _ => None,
        }
    }

    fn next_mapping(curr: char, next: char) -> Option<&'static str> {
        match (curr, next) {
            ('к', 'с') => Some(""),
            _ => None,
        }
    }

    fn ending_mapping(ending: [char; 2]) -> Option<&'static str> {
        match ending {
            ['и', 'н'] => Some("ine"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        assert_eq!(
            Mvd310Fr::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Iouliia, sech echtche etikh miagkikh frantsouzskikh boulok iz Iochkar-Oly, da vypei altaiskogo tchaiou"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(
            Mvd310Fr::transliterate("Юлия Щеглова"),
            "Iouliia Chtcheglova"
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            Mvd310Fr::transliterate("Гайа Васильева"),
            "Gaia Vasilieva"
        );
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(
            Mvd310Fr::transliterate("Андрей Видный"),
            "Andrei Vidnyi"
        );
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(
            Mvd310Fr::transliterate("Оксана Снегирёва"),
            "Oxana Sneguireva"
        );
    }

    #[test]
    fn test_sample_5() {
        assert_eq!(
            Mvd310Fr::transliterate("Юрий Васин"),
            "Iourii Vasine"
        );
    }
}
