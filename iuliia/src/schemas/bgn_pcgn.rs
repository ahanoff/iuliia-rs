//! BGN/PCGN transliteration schema
//!
//! See: <https://dangry.ru/iuliia/bgn-pcgn/>

use crate::Schema;

/// BGN/PCGN transliteration schema.
pub struct BgnPcgn;

impl Schema for BgnPcgn {
    const NAME: &'static str = "bgn_pcgn";

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
            'ъ' => Some("”"),
            'ы' => Some("y"),
            'ь' => Some("’"),
            'э' => Some("e"),
            'ю' => Some("yu"),
            'я' => Some("ya"),
            'ё' => Some("ё"),
            _ => None,
        }
    }

    fn prev_mapping(prev: Option<char>, curr: char) -> Option<&'static str> {
        match (prev, curr) {
            (Some('а'), 'е') => Some("ye"),
            (Some('а'), 'ы') => Some("·y"),
            (Some('а'), 'ё') => Some("yё"),
            (Some('б'), 'э') => Some("·e"),
            (Some('в'), 'э') => Some("·e"),
            (Some('г'), 'э') => Some("·e"),
            (Some('д'), 'э') => Some("·e"),
            (None, 'е') => Some("ye"),
            (Some('е'), 'е') => Some("ye"),
            (Some('е'), 'ы') => Some("·y"),
            (Some('е'), 'ё') => Some("yё"),
            (Some('ж'), 'э') => Some("·e"),
            (Some('з'), 'э') => Some("·e"),
            (Some('и'), 'е') => Some("ye"),
            (Some('и'), 'ы') => Some("·y"),
            (Some('и'), 'ё') => Some("yё"),
            (Some('й'), 'е') => Some("ye"),
            (Some('й'), 'ё') => Some("yё"),
            (Some('к'), 'э') => Some("·e"),
            (Some('л'), 'э') => Some("·e"),
            (Some('м'), 'э') => Some("·e"),
            (Some('н'), 'э') => Some("·e"),
            (Some('о'), 'е') => Some("ye"),
            (Some('о'), 'ы') => Some("·y"),
            (Some('о'), 'ё') => Some("yё"),
            (Some('п'), 'э') => Some("·e"),
            (Some('р'), 'э') => Some("·e"),
            (Some('с'), 'э') => Some("·e"),
            (Some('т'), 'э') => Some("·e"),
            (Some('у'), 'е') => Some("ye"),
            (Some('у'), 'ы') => Some("·y"),
            (Some('у'), 'ё') => Some("yё"),
            (Some('ф'), 'э') => Some("·e"),
            (Some('х'), 'э') => Some("·e"),
            (Some('ц'), 'э') => Some("·e"),
            (Some('ч'), 'э') => Some("·e"),
            (Some('ш'), 'э') => Some("·e"),
            (Some('щ'), 'э') => Some("·e"),
            (Some('ъ'), 'е') => Some("ye"),
            (Some('ъ'), 'ё') => Some("yё"),
            (Some('ы'), 'е') => Some("ye"),
            (Some('ы'), 'ы') => Some("·y"),
            (Some('ы'), 'ё') => Some("yё"),
            (Some('ь'), 'е') => Some("ye"),
            (Some('ь'), 'ё') => Some("yё"),
            (Some('э'), 'е') => Some("ye"),
            (Some('э'), 'ы') => Some("·y"),
            (Some('э'), 'ё') => Some("yё"),
            (Some('ю'), 'е') => Some("ye"),
            (Some('ю'), 'ы') => Some("·y"),
            (Some('ю'), 'ё') => Some("yё"),
            (Some('я'), 'е') => Some("ye"),
            (Some('я'), 'ы') => Some("·y"),
            (Some('я'), 'ё') => Some("yё"),
            (None, 'ё') => Some("yё"),
            (Some('ё'), 'е') => Some("ye"),
            (Some('ё'), 'ы') => Some("·y"),
            (Some('ё'), 'ё') => Some("yё"),
            _ => None,
        }
    }

    fn next_mapping(curr: char, next: char) -> Option<&'static str> {
        match (curr, next) {
            ('й', 'а') => Some("y·"),
            ('й', 'у') => Some("y·"),
            ('й', 'ы') => Some("y·"),
            ('й', 'э') => Some("y·"),
            ('т', 'с') => Some("t·"),
            ('ш', 'ч') => Some("sh·"),
            ('ы', 'а') => Some("y·"),
            ('ы', 'у') => Some("y·"),
            ('ы', 'ы') => Some("y·"),
            ('ы', 'э') => Some("y·"),
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
            BgnPcgn::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Yuliya, s”yesh’ yeshchё etikh myagkikh frantsuzskikh bulok iz Yoshkar-Oly, da vypey altayskogo chayu"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(
            BgnPcgn::transliterate("Россия, город Йошкар-Ола, улица Яна Крастыня"),
            "Rossiya, gorod Yoshkar-Ola, ulitsa Yana Krastynya"
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(BgnPcgn::transliterate("Елизово"), "Yelizovo");
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(BgnPcgn::transliterate("Чапаевск"), "Chapayevsk");
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(BgnPcgn::transliterate("Мейеровка"), "Meyyerovka");
    }

    #[test]
    fn test_sample_5() {
        assert_eq!(BgnPcgn::transliterate("Юрьев объезд"), "Yur’yev ob”yezd");
    }

    #[test]
    fn test_sample_6() {
        assert_eq!(BgnPcgn::transliterate("Белкино"), "Belkino");
    }

    #[test]
    fn test_sample_7() {
        assert_eq!(BgnPcgn::transliterate("Ёдва"), "Yёdva");
    }

    #[test]
    fn test_sample_8() {
        assert_eq!(BgnPcgn::transliterate("Змииёвка"), "Zmiiyёvka");
    }

    #[test]
    fn test_sample_9() {
        assert_eq!(BgnPcgn::transliterate("Айёган"), "Ayyёgan");
    }

    #[test]
    fn test_sample_10() {
        assert_eq!(BgnPcgn::transliterate("Воробьёво"), "Vorob’yёvo");
    }

    #[test]
    fn test_sample_11() {
        assert_eq!(BgnPcgn::transliterate("Кебанъёль"), "Keban”yёl’");
    }

    #[test]
    fn test_sample_12() {
        assert_eq!(BgnPcgn::transliterate("Озёрный"), "Ozёrnyy");
    }

    #[test]
    fn test_sample_13() {
        assert_eq!(BgnPcgn::transliterate("Тыайа"), "Ty·ay·a");
    }

    #[test]
    fn test_sample_14() {
        assert_eq!(BgnPcgn::transliterate("Сайылык"), "Say·ylyk");
    }

    #[test]
    fn test_sample_15() {
        assert_eq!(BgnPcgn::transliterate("Ойусардах"), "Oy·usardakh");
    }

    #[test]
    fn test_sample_16() {
        assert_eq!(BgnPcgn::transliterate("Йошкар-Ола"), "Yoshkar-Ola");
    }

    #[test]
    fn test_sample_17() {
        assert_eq!(BgnPcgn::transliterate("Бийск"), "Biysk");
    }

    #[test]
    fn test_sample_18() {
        assert_eq!(BgnPcgn::transliterate("Тыэкан"), "Ty·ekan");
    }

    #[test]
    fn test_sample_19() {
        assert_eq!(BgnPcgn::transliterate("Суык-Су"), "Su·yk-Su");
    }

    #[test]
    fn test_sample_20() {
        assert_eq!(BgnPcgn::transliterate("Тында"), "Tynda");
    }

    #[test]
    fn test_sample_21() {
        assert_eq!(BgnPcgn::transliterate("Улан-Удэ"), "Ulan-Ud·e");
    }

    #[test]
    fn test_sample_22() {
        assert_eq!(BgnPcgn::transliterate("Электрогорск"), "Elektrogorsk");
    }

    #[test]
    fn test_sample_23() {
        assert_eq!(BgnPcgn::transliterate("Руэм"), "Ruem");
    }

    #[test]
    fn test_sample_24() {
        assert_eq!(BgnPcgn::transliterate("Вяртсиля"), "Vyart·silya");
    }

    #[test]
    fn test_sample_25() {
        assert_eq!(BgnPcgn::transliterate("Ташчишма"), "Tash·chishma");
    }
}
