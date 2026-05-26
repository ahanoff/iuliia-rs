//! GOST R 7.0.34-2014 transliteration schema
//!
//! See: <http://localhost:3000/iuliia/gost-7034/>
//!
//! This schema defines alternatives for many letters, but does not specify when to use which:
//!   е → e  (ye)
//!   ё → yo (jo)
//!   й → j  (i,y)
//!   х → x  (kh)
//!   ц → c  (tz,cz)
//!   ъ → '' (empty)
//!   ь → '  (empty)
//!   ю → yu (ju)
//!   я → ya (ja)
//! `iuliia` uses the first of suggested translations for each such letter.

use crate::Schema;

/// GOST R 7.0.34-2014 transliteration schema.
pub struct Gost7034;

impl Schema for Gost7034 {
    const NAME: &'static str = "gost_7034";

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
            'ц' => Some("c"),
            'ч' => Some("ch"),
            'ш' => Some("sh"),
            'щ' => Some("shh"),
            'ъ' => Some("''"),
            'ы' => Some("y"),
            'ь' => Some("'"),
            'э' => Some("e"),
            'ю' => Some("yu"),
            'я' => Some("ya"),
            'ё' => Some("yo"),
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
            Gost7034::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Yuliya, s''esh' eshhyo etix myagkix francuzskix bulok iz Joshkar-Oly, da vypej altajskogo chayu"
        );
    }
}
