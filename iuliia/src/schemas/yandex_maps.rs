//! Yandex.Maps transliteration schema
//!
//! See: <https://dangry.ru/iuliia/yandex-maps/>

use crate::Schema;

/// Yandex.Maps transliteration schema.
pub struct YandexMaps;

impl Schema for YandexMaps {
    const NAME: &'static str = "yandex_maps";

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
            'щ' => Some("sch"),
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

    fn prev_mapping(_prev: Option<char>, _curr: char) -> Option<&'static str> {
        None
    }

    fn next_mapping(curr: char, next: char) -> Option<&'static str> {
        match (curr, next) {
            ('ъ', 'е') => Some("y"),
            _ => None,
        }
    }

    fn ending_mapping(ending: [char; 2]) -> Option<&'static str> {
        match ending {
            ['ы', 'й'] => Some("iy"),
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
            YandexMaps::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Yuliya, syesh eschyo etikh myagkikh frantsuzskikh bulok iz Yoshkar-Oly, da vypey altayskogo chayu"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(
            YandexMaps::transliterate("Россия, город Йошкар-Ола, улица Яна Крастыня"),
            "Rossiya, gorod Yoshkar-Ola, ulitsa Yana Krastynya"
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            YandexMaps::transliterate("Санкт-Петербург, Подъездной пер"),
            "Sankt-Peterburg, Podyezdnoy per"
        );
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(
            YandexMaps::transliterate("Москва, ул Подъёмная"),
            "Moskva, ul Podyomnaya"
        );
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(
            YandexMaps::transliterate("Астрахань, ул Подъяпольского"),
            "Astrakhan, ul Podyapolskogo"
        );
    }

    #[test]
    fn test_sample_5() {
        assert_eq!(YandexMaps::transliterate("Щегловитовка"), "Scheglovitovka");
    }

    #[test]
    fn test_sample_6() {
        assert_eq!(YandexMaps::transliterate("Новый Уренгой"), "Noviy Urengoy");
    }
}
