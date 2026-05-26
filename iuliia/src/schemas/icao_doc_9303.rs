//! ICAO DOC 9303 transliteration schema
//!
//! See: <https://dangry.ru/iuliia/icao-doc-9303/>

use crate::Schema;

/// ICAO DOC 9303 transliteration schema.
pub struct IcaoDoc9303;

impl Schema for IcaoDoc9303 {
    const NAME: &'static str = "icao_doc_9303";

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
            'щ' => Some("shch"),
            'ъ' => Some("ie"),
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
            IcaoDoc9303::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Iuliia, sieesh eshche etikh miagkikh frantsuzskikh bulok iz Ioshkar-Oly, da vypei altaiskogo chaiu"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(
            IcaoDoc9303::transliterate("Юлия Щеглова"),
            "Iuliia Shcheglova"
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            IcaoDoc9303::transliterate("Гайа Васильева"),
            "Gaia Vasileva"
        );
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(IcaoDoc9303::transliterate("Андрей Видный"), "Andrei Vidnyi");
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(IcaoDoc9303::transliterate("Артём Краевой"), "Artem Kraevoi");
    }

    #[test]
    fn test_sample_5() {
        assert_eq!(IcaoDoc9303::transliterate("Мадыр Чёткий"), "Madyr Chetkii");
    }

    #[test]
    fn test_sample_6() {
        assert_eq!(
            IcaoDoc9303::transliterate("Оксана Клеёнкина"),
            "Oksana Kleenkina"
        );
    }

    #[test]
    fn test_sample_7() {
        assert_eq!(IcaoDoc9303::transliterate("Игорь Ильин"), "Igor Ilin");
    }

    #[test]
    fn test_sample_8() {
        assert_eq!(
            IcaoDoc9303::transliterate("Ян Разъездной"),
            "Ian Razieezdnoi"
        );
    }
}
