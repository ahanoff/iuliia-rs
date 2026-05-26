//! Yandex.Money transliteration schema
//!
//! See: <https://dangry.ru/iuliia/yandex-money/>

use crate::Schema;

/// Yandex.Money transliteration schema.
pub struct YandexMoney;

impl Schema for YandexMoney {
    const NAME: &'static str = "yandex_money";

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
            'ё' => Some("e"),
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
            YandexMoney::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Yuliya, sesh esche etikh myagkikh frantsuzskikh bulok iz Ioshkar-Oly, da vypei altaiskogo chayu"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(
            YandexMoney::transliterate("Юлия Щеглова"),
            "Yuliya Scheglova"
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            YandexMoney::transliterate("Иван Брызгальский"),
            "Ivan Bryzgalskii"
        );
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(YandexMoney::transliterate("Ксения Стрый"), "Kseniya Stryi");
    }
}
