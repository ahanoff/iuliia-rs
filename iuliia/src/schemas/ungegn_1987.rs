//! UNGEGN 1987 V/18 transliteration schema
//!
//! See: <https://dangry.ru/iuliia/ungegn-1987/>

use crate::Schema;

/// UNGEGN 1987 V/18 transliteration schema.
pub struct Ungegn1987;

impl Schema for Ungegn1987 {
    const NAME: &'static str = "ungegn_1987";

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
            'щ' => Some("šč"),
            'ъ' => Some("ʺ"),
            'ы' => Some("y"),
            'ь' => Some("ʹ"),
            'э' => Some("è"),
            'ю' => Some("ju"),
            'я' => Some("ja"),
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
            Ungegn1987::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Julija, sʺešʹ eščё ètih mjagkih francuzskih bulok iz Joškar-Oly, da vypej altajskogo čaju"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(
            Ungegn1987::transliterate("Россия, город Йошкар-Ола, улица Яна Крастыня"),
            "Rossija, gorod Joškar-Ola, ulica Jana Krastynja"
        );
    }
}
