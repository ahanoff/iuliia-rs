//! ALA-LC transliteration schema.
//!
//! See: <https://dangry.ru/iuliia/ala-lc/>

use crate::Schema;

/// ALA-LC transliteration schema..
pub struct AlaLc;

impl Schema for AlaLc {
    const NAME: &'static str = "ala_lc";

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
            'й' => Some("ĭ"),
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
            'ц' => Some("t\u{361}s"),
            'ч' => Some("ch"),
            'ш' => Some("sh"),
            'щ' => Some("shch"),
            'ъ' => Some("ʺ"),
            'ы' => Some("y"),
            'ь' => Some("ʹ"),
            'э' => Some("ė"),
            'ю' => Some("i\u{361}u"),
            'я' => Some("i\u{361}a"),
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
            AlaLc::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "I\u{361}ulii\u{361}a, sʺeshʹ eshchё ėtikh mi\u{361}agkikh frant\u{361}suzskikh bulok iz Ĭoshkar-Oly, da vypeĭ altaĭskogo chai\u{361}u"
        );
    }
}
