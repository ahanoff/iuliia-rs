//! BGN/PCGN transliteration schema
//!
//! See: <https://dangry.ru/iuliia/bgn-pcgn/>

use crate::Schema;

/// BGN/PCGN transliteration schema.
pub struct BgnPcgnAlt;

impl Schema for BgnPcgnAlt {
    const NAME: &'static str = "bgn_pcgn_alt";

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
            (Some('а'), 'ё') => Some("yё"),
            (None, 'е') => Some("ye"),
            (Some('е'), 'е') => Some("ye"),
            (Some('е'), 'ё') => Some("yё"),
            (Some('и'), 'е') => Some("ye"),
            (Some('и'), 'ё') => Some("yё"),
            (Some('й'), 'е') => Some("ye"),
            (Some('й'), 'ё') => Some("yё"),
            (Some('о'), 'е') => Some("ye"),
            (Some('о'), 'ё') => Some("yё"),
            (Some('у'), 'е') => Some("ye"),
            (Some('у'), 'ё') => Some("yё"),
            (Some('ъ'), 'е') => Some("ye"),
            (Some('ъ'), 'ё') => Some("yё"),
            (Some('ы'), 'е') => Some("ye"),
            (Some('ы'), 'ё') => Some("yё"),
            (Some('ь'), 'е') => Some("ye"),
            (Some('ь'), 'ё') => Some("yё"),
            (Some('э'), 'е') => Some("ye"),
            (Some('э'), 'ё') => Some("yё"),
            (Some('ю'), 'е') => Some("ye"),
            (Some('ю'), 'ё') => Some("yё"),
            (Some('я'), 'е') => Some("ye"),
            (Some('я'), 'ё') => Some("yё"),
            (None, 'ё') => Some("yё"),
            (Some('ё'), 'е') => Some("ye"),
            (Some('ё'), 'ё') => Some("yё"),
            _ => None,
        }
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
            BgnPcgnAlt::transliterate("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"),
            "Yuliya, s”yesh’ yeshchё etikh myagkikh frantsuzskikh bulok iz Yoshkar-Oly, da vypey altayskogo chayu"
        );
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(
            BgnPcgnAlt::transliterate("Россия, город Йошкар-Ола, улица Яна Крастыня"),
            "Rossiya, gorod Yoshkar-Ola, ulitsa Yana Krastynya"
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(BgnPcgnAlt::transliterate("Елизово"), "Yelizovo");
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(BgnPcgnAlt::transliterate("Чапаевск"), "Chapayevsk");
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(BgnPcgnAlt::transliterate("Мейеровка"), "Meyyerovka");
    }

    #[test]
    fn test_sample_5() {
        assert_eq!(BgnPcgnAlt::transliterate("Юрьев объезд"), "Yur’yev ob”yezd");
    }

    #[test]
    fn test_sample_6() {
        assert_eq!(BgnPcgnAlt::transliterate("Белкино"), "Belkino");
    }

    #[test]
    fn test_sample_7() {
        assert_eq!(BgnPcgnAlt::transliterate("Ёдва"), "Yёdva");
    }

    #[test]
    fn test_sample_8() {
        assert_eq!(BgnPcgnAlt::transliterate("Змииёвка"), "Zmiiyёvka");
    }

    #[test]
    fn test_sample_9() {
        assert_eq!(BgnPcgnAlt::transliterate("Айёган"), "Ayyёgan");
    }

    #[test]
    fn test_sample_10() {
        assert_eq!(BgnPcgnAlt::transliterate("Воробьёво"), "Vorob’yёvo");
    }

    #[test]
    fn test_sample_11() {
        assert_eq!(BgnPcgnAlt::transliterate("Кебанъёль"), "Keban”yёl’");
    }

    #[test]
    fn test_sample_12() {
        assert_eq!(BgnPcgnAlt::transliterate("Озёрный"), "Ozёrnyy");
    }

    #[test]
    fn test_sample_13() {
        assert_eq!(BgnPcgnAlt::transliterate("Тыайа"), "Tyaya");
    }

    #[test]
    fn test_sample_14() {
        assert_eq!(BgnPcgnAlt::transliterate("Сайылык"), "Sayylyk");
    }

    #[test]
    fn test_sample_15() {
        assert_eq!(BgnPcgnAlt::transliterate("Ойусардах"), "Oyusardakh");
    }

    #[test]
    fn test_sample_16() {
        assert_eq!(BgnPcgnAlt::transliterate("Йошкар-Ола"), "Yoshkar-Ola");
    }

    #[test]
    fn test_sample_17() {
        assert_eq!(BgnPcgnAlt::transliterate("Бийск"), "Biysk");
    }

    #[test]
    fn test_sample_18() {
        assert_eq!(BgnPcgnAlt::transliterate("Тыэкан"), "Tyekan");
    }

    #[test]
    fn test_sample_19() {
        assert_eq!(BgnPcgnAlt::transliterate("Суык-Су"), "Suyk-Su");
    }

    #[test]
    fn test_sample_20() {
        assert_eq!(BgnPcgnAlt::transliterate("Тында"), "Tynda");
    }

    #[test]
    fn test_sample_21() {
        assert_eq!(BgnPcgnAlt::transliterate("Улан-Удэ"), "Ulan-Ude");
    }

    #[test]
    fn test_sample_22() {
        assert_eq!(BgnPcgnAlt::transliterate("Электрогорск"), "Elektrogorsk");
    }

    #[test]
    fn test_sample_23() {
        assert_eq!(BgnPcgnAlt::transliterate("Руэм"), "Ruem");
    }

    #[test]
    fn test_sample_24() {
        assert_eq!(BgnPcgnAlt::transliterate("Вяртсиля"), "Vyartsilya");
    }

    #[test]
    fn test_sample_25() {
        assert_eq!(BgnPcgnAlt::transliterate("Ташчишма"), "Tashchishma");
    }
}
