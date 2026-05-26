//! Wikipedia transliteration schema
//!
//! See: <https://dangry.ru/iuliia/wikipedia/>

use crate::Schema;

/// Wikipedia transliteration schema.
pub struct Wikipedia;

impl Schema for Wikipedia {
    const NAME: &'static str = "wikipedia";

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
            'ъ' => Some(""),
            'ы' => Some("y"),
            'ь' => Some(""),
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
            (None, 'е') => Some("ye"),
            (Some('и'), 'е') => Some("ye"),
            (Some('о'), 'е') => Some("ye"),
            (Some('у'), 'е') => Some("ye"),
            (Some('ъ'), 'е') => Some("ye"),
            (Some('ь'), 'е') => Some("ye"),
            (Some('э'), 'е') => Some("ye"),
            (Some('ю'), 'е') => Some("ye"),
            (Some('я'), 'е') => Some("ye"),
            _ => None,
        }
    }

    fn next_mapping(curr: char, next: char) -> Option<&'static str> {
        match (curr, next) {
            ('ъ', 'а') => Some("y"),
            ('ъ', 'и') => Some("y"),
            ('ъ', 'о') => Some("y"),
            ('ъ', 'у') => Some("y"),
            ('ъ', 'ы') => Some("y"),
            ('ъ', 'э') => Some("y"),
            ('ь', 'а') => Some("y"),
            ('ь', 'и') => Some("y"),
            ('ь', 'о') => Some("y"),
            ('ь', 'у') => Some("y"),
            ('ь', 'ы') => Some("y"),
            ('ь', 'э') => Some("y"),
            _ => None,
        }
    }

    fn ending_mapping(ending: [char; 2]) -> Option<&'static str> {
        match ending {
            ['и', 'й'] => Some("y"),
            ['ы', 'й'] => Some("y"),
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
            Wikipedia::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Yuliya, syesh yeshchyo etikh myagkikh frantsuzskikh bulok iz Yoshkar-Oly, da vypey altayskogo chayu"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(
            Wikipedia::transliterate("Россия, город Йошкар-Ола, улица Яна Крастыня"),
            "Rossiya, gorod Yoshkar-Ola, ulitsa Yana Krastynya"
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(Wikipedia::transliterate("Ельцин"), "Yeltsin");
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(Wikipedia::transliterate("Раздольное"), "Razdolnoye");
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(Wikipedia::transliterate("Юрьев"), "Yuryev");
    }

    #[test]
    fn test_sample_5() {
        assert_eq!(Wikipedia::transliterate("Белкин"), "Belkin");
    }

    #[test]
    fn test_sample_6() {
        assert_eq!(Wikipedia::transliterate("Бийск"), "Biysk");
    }

    #[test]
    fn test_sample_7() {
        assert_eq!(Wikipedia::transliterate("Подъярский"), "Podyarsky");
    }

    #[test]
    fn test_sample_8() {
        assert_eq!(
            Wikipedia::transliterate("Мусийкъонгийкоте"),
            "Musiykyongiykote"
        );
    }

    #[test]
    fn test_sample_9() {
        assert_eq!(Wikipedia::transliterate("Давыдов"), "Davydov");
    }

    #[test]
    fn test_sample_10() {
        assert_eq!(Wikipedia::transliterate("Усолье"), "Usolye");
    }

    #[test]
    fn test_sample_11() {
        assert_eq!(Wikipedia::transliterate("Выхухоль"), "Vykhukhol");
    }

    #[test]
    fn test_sample_12() {
        assert_eq!(Wikipedia::transliterate("Дальнегорск"), "Dalnegorsk");
    }

    #[test]
    fn test_sample_13() {
        assert_eq!(Wikipedia::transliterate("Ильинский"), "Ilyinsky");
    }

    #[test]
    fn test_sample_14() {
        assert_eq!(Wikipedia::transliterate("Красный"), "Krasny");
    }

    #[test]
    fn test_sample_15() {
        assert_eq!(Wikipedia::transliterate("Великий"), "Veliky");
    }

    #[test]
    fn test_sample_16() {
        assert_eq!(
            Wikipedia::transliterate("Набережные Челны"),
            "Naberezhnye Chelny"
        );
    }
}
