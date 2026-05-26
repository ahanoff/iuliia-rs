//! MVD 310-1997 transliteration schema
//!
//! See: <https://dangry.ru/iuliia/mvd-310/>

use crate::Schema;

/// MVD 310-1997 transliteration schema.
pub struct Mvd310;

impl Schema for Mvd310 {
    const NAME: &'static str = "mvd_310";

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
            'й' => Some("y"),
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
            'ю' => Some("yu"),
            'я' => Some("ya"),
            'ё' => Some("e"),
            _ => None,
        }
    }

    fn prev_mapping(prev: Option<char>, curr: char) -> Option<&'static str> {
        match (prev, curr) {
            (Some('ъ'), 'е') => Some("ye"),
            (Some('ь'), 'е') => Some("ye"),
            _ => None,
        }
    }

    fn next_mapping(curr: char, next: char) -> Option<&'static str> {
        match (curr, next) {
            ('ъ', 'е') => Some(""),
            ('ь', 'е') => Some(""),
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
            Mvd310::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Yuliya, syesh' eshche etikh myagkikh frantsuzskikh bulok iz Yoshkar-Oly, da vypey altayskogo chayu"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(Mvd310::transliterate("Юлия Щеглова"), "Yuliya Shcheglova");
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(Mvd310::transliterate("Гайа Васильева"), "Gaya Vasilyeva");
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(Mvd310::transliterate("Андрей Видный"), "Andrey Vidnyy");
    }
}
