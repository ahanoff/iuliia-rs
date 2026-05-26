//! MVD 782-2000 transliteration schema
//!
//! See: <https://dangry.ru/iuliia/mvd-782/>

use crate::Schema;

/// MVD 782-2000 transliteration schema.
pub struct Mvd782;

impl Schema for Mvd782 {
    const NAME: &'static str = "mvd_782";

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
            'ъ' => Some("'"),
            'ы' => Some("y"),
            'ь' => Some("'"),
            'э' => Some("e"),
            'ю' => Some("yu"),
            'я' => Some("ya"),
            'ё' => Some("yo"),
            _ => None,
        }
    }

    fn prev_mapping(prev: Option<char>, curr: char) -> Option<&'static str> {
        match (prev, curr) {
            (Some('а'), 'е') => Some("ye"),
            (Some('б'), 'ё') => Some("ye"),
            (Some('в'), 'ё') => Some("ye"),
            (Some('г'), 'ё') => Some("ye"),
            (Some('д'), 'ё') => Some("ye"),
            (Some('е'), 'е') => Some("ye"),
            (Some('ж'), 'ё') => Some("e"),
            (Some('з'), 'ё') => Some("ye"),
            (Some('и'), 'е') => Some("ye"),
            (Some('к'), 'ё') => Some("ye"),
            (Some('л'), 'ё') => Some("ye"),
            (Some('м'), 'ё') => Some("ye"),
            (Some('н'), 'ё') => Some("ye"),
            (Some('о'), 'е') => Some("ye"),
            (Some('п'), 'ё') => Some("ye"),
            (Some('р'), 'ё') => Some("ye"),
            (Some('с'), 'ё') => Some("ye"),
            (Some('т'), 'ё') => Some("ye"),
            (Some('у'), 'е') => Some("ye"),
            (Some('ф'), 'ё') => Some("ye"),
            (Some('х'), 'ё') => Some("ye"),
            (Some('ц'), 'ё') => Some("ye"),
            (Some('ч'), 'ё') => Some("e"),
            (Some('ш'), 'ё') => Some("e"),
            (Some('щ'), 'ё') => Some("e"),
            (Some('ъ'), 'е') => Some("ye"),
            (Some('ы'), 'е') => Some("ye"),
            (Some('ь'), 'е') => Some("ye"),
            (Some('ь'), 'и') => Some("yi"),
            (Some('э'), 'е') => Some("ye"),
            (Some('ю'), 'е') => Some("ye"),
            (Some('я'), 'е') => Some("ye"),
            (Some('ё'), 'е') => Some("ye"),
            _ => None,
        }
    }

    fn next_mapping(curr: char, next: char) -> Option<&'static str> {
        match (curr, next) {
            ('ъ', 'е') => Some(""),
            ('ъ', 'ё') => Some(""),
            ('ь', 'е') => Some(""),
            ('ь', 'и') => Some(""),
            ('ь', 'ё') => Some(""),
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
            Mvd782::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Yuliya, syesh' eshche etikh myagkikh frantsuzskikh bulok iz Yoshkar-Oly, da vypey altayskogo chayu"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(
            Mvd782::transliterate("Юлия Щеглова"),
            "Yuliya Shcheglova"
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            Mvd782::transliterate("Гайа Васильева"),
            "Gaya Vasilyeva"
        );
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(
            Mvd782::transliterate("Андрей Видный"),
            "Andrey Vidnyy"
        );
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(
            Mvd782::transliterate("Артём Краевой"),
            "Artyem Krayevoy"
        );
    }

    #[test]
    fn test_sample_5() {
        assert_eq!(
            Mvd782::transliterate("Мадыр Чёткий"),
            "Madyr Chetkiy"
        );
    }

    #[test]
    fn test_sample_6() {
        assert_eq!(
            Mvd782::transliterate("Оксана Клеёнкина"),
            "Oksana Kleyonkina"
        );
    }

    #[test]
    fn test_sample_7() {
        assert_eq!(
            Mvd782::transliterate("Игорь Ильин"),
            "Igor' Ilyin"
        );
    }
}
