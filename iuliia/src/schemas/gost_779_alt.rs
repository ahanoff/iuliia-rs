//! GOST 7.79-2000 (aka ISO 9:1995) transliteration schema
//!
//! See: <https://dangry.ru/iuliia/gost-779/>

use crate::Schema;

/// GOST 7.79-2000 (aka ISO 9:1995) transliteration schema.
pub struct Gost779Alt;

impl Schema for Gost779Alt {
    const NAME: &'static str = "gost_779_alt";

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
            'х' => Some("x"),
            'ц' => Some("cz"),
            'ч' => Some("ch"),
            'ш' => Some("sh"),
            'щ' => Some("shh"),
            'ъ' => Some("``"),
            'ы' => Some("y`"),
            'ь' => Some("`"),
            'э' => Some("е`"),
            'ю' => Some("yu"),
            'я' => Some("ya"),
            'ё' => Some("yo"),
            _ => None,
        }
    }

    fn prev_mapping(_prev: Option<char>, _curr: char) -> Option<&'static str> {
        None
    }

    fn next_mapping(curr: char, next: char) -> Option<&'static str> {
        match (curr, next) {
            ('ц', 'е') => Some("c"),
            ('ц', 'и') => Some("c"),
            ('ц', 'й') => Some("c"),
            ('ц', 'ы') => Some("c"),
            _ => None,
        }
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
            Gost779Alt::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Yuliya, s``esh` eshhyo е`tix myagkix franczuzskix bulok iz Joshkar-Oly`, da vy`pej altajskogo chayu"
        );
    }
}
