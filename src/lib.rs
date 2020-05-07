pub mod iuliia {
    use std::collections::HashMap;

    pub trait Transliterator {
        fn transliterate(&self, input: String) -> String;
    }

    pub struct IcaoDoc9303 {
        mapping: HashMap<String, String>
    }
    impl IcaoDoc9303 {
        pub fn new() -> Self {
            let mut mapping= HashMap::new();

            mapping.insert("а".to_string(), "a".to_string());
            mapping.insert("б".to_string(), "b".to_string());
            mapping.insert("в".to_string(), "v".to_string());
            mapping.insert("г".to_string(), "g".to_string());
            mapping.insert("д".to_string(), "d".to_string());
            mapping.insert("е".to_string(), "e".to_string());
            mapping.insert("ё".to_string(), "e".to_string());
            mapping.insert("ж".to_string(), "zh".to_string());
            mapping.insert("з".to_string(), "z".to_string());
            mapping.insert("и".to_string(), "i".to_string());
            mapping.insert("й".to_string(), "i".to_string());
            mapping.insert("к".to_string(), "k".to_string());
            mapping.insert("л".to_string(), "l".to_string());
            mapping.insert("м".to_string(), "m".to_string());
            mapping.insert("н".to_string(), "n".to_string());
            mapping.insert("о".to_string(), "o".to_string());
            mapping.insert("п".to_string(), "p".to_string());
            mapping.insert("р".to_string(), "r".to_string());
            mapping.insert("с".to_string(), "s".to_string());
            mapping.insert("т".to_string(), "t".to_string());
            mapping.insert("у".to_string(), "u".to_string());
            mapping.insert("ф".to_string(), "f".to_string());
            mapping.insert("х".to_string(), "kh".to_string());
            mapping.insert("ц".to_string(), "ts".to_string());
            mapping.insert("ч".to_string(), "ch".to_string());
            mapping.insert("ш".to_string(), "sh".to_string());
            mapping.insert("щ".to_string(), "shch".to_string());
            mapping.insert("ъ".to_string(), "ie".to_string());
            mapping.insert("ы".to_string(), "y".to_string());
            mapping.insert("ь".to_string(), "".to_string());
            mapping.insert("э".to_string(), "e".to_string());
            mapping.insert("ю".to_string(), "iu".to_string());
            mapping.insert("я".to_string(), "ia".to_string());


            IcaoDoc9303 {
                mapping
            }
        }
    }

    impl Transliterator for IcaoDoc9303 {
        fn transliterate(&self, input: String) -> String {
            input
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iuliia::Transliterator;

    #[test]
    fn first() {
        let w = crate::iuliia::IcaoDoc9303::new();
        let o = w.transliterate("Юлия Щеглова".to_string());
        assert_eq!(o, "Iuliia Shcheglova".to_string());
    }
}
