//! Moscow Metro map transliteration schema
//!
//! See: <https://dangry.ru/iuliia/mosmetro/>

use crate::Schema;

/// Moscow Metro map transliteration schema.
pub struct Mosmetro;

impl Schema for Mosmetro {
    const NAME: &'static str = "mosmetro";

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
            'ё' => Some("e"),
            _ => None,
        }
    }

    fn prev_mapping(prev: Option<char>, curr: char) -> Option<&'static str> {
        match (prev, curr) {
            (Some('т'), 'ц') => Some("s"),
            (Some('ъ'), 'ё') => Some("o"),
            (Some('ь'), 'ё') => Some("o"),
            _ => None,
        }
    }

    fn next_mapping(curr: char, next: char) -> Option<&'static str> {
        match (curr, next) {
            ('ъ', 'а') => Some("y"),
            ('ъ', 'е') => Some("y"),
            ('ъ', 'и') => Some("y"),
            ('ъ', 'о') => Some("y"),
            ('ъ', 'у') => Some("y"),
            ('ъ', 'э') => Some("y"),
            ('ъ', 'ё') => Some("y"),
            ('ь', 'а') => Some("y"),
            ('ь', 'е') => Some("y"),
            ('ь', 'и') => Some("y"),
            ('ь', 'о') => Some("y"),
            ('ь', 'у') => Some("y"),
            ('ь', 'э') => Some("y"),
            ('ь', 'ё') => Some("y"),
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
            Mosmetro::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Yuliya, syesh esche etikh myagkikh frantsuzskikh bulok iz Yoshkar-Oly, da vypey altayskogo chayu"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(Mosmetro::transliterate("Битцевский парк"), "Bitsevsky park");
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            Mosmetro::transliterate("Верхние Лихоборы"),
            "Verkhnie Likhobory"
        );
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(Mosmetro::transliterate("Воробьёвы горы"), "Vorobyovy gory");
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(Mosmetro::transliterate("Выхино"), "Vykhino");
    }

    #[test]
    fn test_sample_5() {
        assert_eq!(Mosmetro::transliterate("Зябликово"), "Zyablikovo");
    }

    #[test]
    fn test_sample_6() {
        assert_eq!(Mosmetro::transliterate("Измайловская"), "Izmaylovskaya");
    }

    #[test]
    fn test_sample_7() {
        assert_eq!(Mosmetro::transliterate("Кожуховская"), "Kozhukhovskaya");
    }

    #[test]
    fn test_sample_8() {
        assert_eq!(Mosmetro::transliterate("Крылатское"), "Krylatskoe");
    }

    #[test]
    fn test_sample_9() {
        assert_eq!(Mosmetro::transliterate("Марьина Роща"), "Maryina Roscha");
    }

    #[test]
    fn test_sample_10() {
        assert_eq!(Mosmetro::transliterate("Марьино"), "Maryino");
    }

    #[test]
    fn test_sample_11() {
        assert_eq!(Mosmetro::transliterate("Молодёжная"), "Molodezhnaya");
    }

    #[test]
    fn test_sample_12() {
        assert_eq!(Mosmetro::transliterate("Октябрьская"), "Oktyabrskaya");
    }

    #[test]
    fn test_sample_13() {
        assert_eq!(Mosmetro::transliterate("Ольховая"), "Olkhovaya");
    }

    #[test]
    fn test_sample_14() {
        assert_eq!(Mosmetro::transliterate("Парк Победы"), "Park Pobedy");
    }

    #[test]
    fn test_sample_15() {
        assert_eq!(
            Mosmetro::transliterate("Площадь Ильича"),
            "Ploschad Ilyicha"
        );
    }

    #[test]
    fn test_sample_16() {
        assert_eq!(
            Mosmetro::transliterate("Площадь Революции"),
            "Ploschad Revolyutsii"
        );
    }

    #[test]
    fn test_sample_17() {
        assert_eq!(
            Mosmetro::transliterate("Пятницкое шоссе"),
            "Pyatnitskoe shosse"
        );
    }

    #[test]
    fn test_sample_18() {
        assert_eq!(Mosmetro::transliterate("Румянцево"), "Rumyantsevo");
    }

    #[test]
    fn test_sample_19() {
        assert_eq!(Mosmetro::transliterate("Саларьево"), "Salaryevo");
    }

    #[test]
    fn test_sample_20() {
        assert_eq!(Mosmetro::transliterate("Семёновская"), "Semenovskaya");
    }

    #[test]
    fn test_sample_21() {
        assert_eq!(Mosmetro::transliterate("Сходненская"), "Skhodnenskaya");
    }

    #[test]
    fn test_sample_22() {
        assert_eq!(Mosmetro::transliterate("Текстильщики"), "Tekstilschiki");
    }

    #[test]
    fn test_sample_23() {
        assert_eq!(Mosmetro::transliterate("Тёплый стан"), "Teply stan");
    }

    #[test]
    fn test_sample_24() {
        assert_eq!(Mosmetro::transliterate("Третьяковская"), "Tretyakovskaya");
    }

    #[test]
    fn test_sample_25() {
        assert_eq!(Mosmetro::transliterate("Тропарёво"), "Troparevo");
    }

    #[test]
    fn test_sample_26() {
        assert_eq!(Mosmetro::transliterate("Фонвизинская"), "Fonvizinskaya");
    }

    #[test]
    fn test_sample_27() {
        assert_eq!(Mosmetro::transliterate("Чистые пруды"), "Chistye prudy");
    }

    #[test]
    fn test_sample_28() {
        assert_eq!(
            Mosmetro::transliterate("Шоссе Энтузиастов"),
            "Shosse Entuziastov"
        );
    }

    #[test]
    fn test_sample_29() {
        assert_eq!(Mosmetro::transliterate("Щёлковская"), "Schelkovskaya");
    }

    #[test]
    fn test_sample_30() {
        assert_eq!(
            Mosmetro::transliterate("Электрозаводская"),
            "Elektrozavodskaya"
        );
    }

    #[test]
    fn test_sample_31() {
        assert_eq!(Mosmetro::transliterate("Юго-Западная"), "Yugo-Zapadnaya");
    }
}
