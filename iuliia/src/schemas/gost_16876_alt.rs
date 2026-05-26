//! GOST 16876-71 (aka GOST 1983) transliteration schema
//!
//! See: <https://dangry.ru/iuliia/gost-16876/>

use crate::Schema;

/// GOST 16876-71 (aka GOST 1983) transliteration schema.
pub struct Gost16876Alt;

impl Schema for Gost16876Alt {
    const NAME: &'static str = "gost_16876_alt";

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
            'й' => Some("jj"),
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
            'ц' => Some("c"),
            'ч' => Some("ch"),
            'ш' => Some("sh"),
            'щ' => Some("shh"),
            'ъ' => Some("\""),
            'ы' => Some("y"),
            'ь' => Some("'"),
            'э' => Some("eh"),
            'ю' => Some("ju"),
            'я' => Some("ja"),
            'ё' => Some("jo"),
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
            Gost16876Alt::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Julija, s\"esh' eshhjo ehtikh mjagkikh francuzskikh bulok iz Jjoshkar-Oly, da vypejj altajjskogo chaju"
        );
    }
}
