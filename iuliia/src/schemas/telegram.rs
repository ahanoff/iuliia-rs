//! Telegram transliteration schema
//!
//! See: <https://dangry.ru/iuliia/telegram/>

use crate::Schema;

/// Telegram transliteration schema.
pub struct Telegram;

impl Schema for Telegram {
    const NAME: &'static str = "telegram";

    fn mapping(c: char) -> Option<&'static str> {
        match c {
            'а' => Some("a"),
            'б' => Some("b"),
            'в' => Some("v"),
            'г' => Some("g"),
            'д' => Some("d"),
            'е' => Some("e"),
            'ж' => Some("j"),
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
            'х' => Some("h"),
            'ц' => Some("c"),
            'ч' => Some("ch"),
            'ш' => Some("sh"),
            'щ' => Some("sc"),
            'ъ' => Some(""),
            'ы' => Some("y"),
            'ь' => Some(""),
            'э' => Some("e"),
            'ю' => Some("iu"),
            'я' => Some("ia"),
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
            Telegram::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Iuliia, sesh esce etih miagkih francuzskih bulok iz Ioshkar-Oly, da vypei altaiskogo chaiu"
        );
    }
}
