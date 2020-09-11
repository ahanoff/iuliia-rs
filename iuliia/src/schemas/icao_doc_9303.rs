use crate::Transliterator;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use std::iter::Map;

pub struct IcaoDoc9303 {
    pub name: String,
    pub description: String,
    pub url: String,
    pub comments: Vec<String>,
    pub mapping: HashMap<String, String>,
    pub prev_mapping: Option<HashMap<String, String>>,
    pub next_mapping: Option<HashMap<String, String>>,
    pub ending_mapping: Option<HashMap<String, String>>,
    pub samples: Vec<(String, String)>
}

impl IcaoDoc9303 {

    fn translate_word(&self, input: &str) -> String {
        return "".to_string();
    }

    fn translate_letter(&self, prev: &str, current: &str, next: &str) -> String {
        return "".to_string();
    }
}

impl Default for IcaoDoc9303 {
    fn default() -> Self {
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
        mapping.insert("А".to_string(), "A".to_string());
        mapping.insert("Б".to_string(), "B".to_string());
        mapping.insert("В".to_string(), "V".to_string());
        mapping.insert("Г".to_string(), "G".to_string());
        mapping.insert("Д".to_string(), "D".to_string());
        mapping.insert("Е".to_string(), "E".to_string());
        mapping.insert("Ё".to_string(), "E".to_string());
        mapping.insert("Ж".to_string(), "Zh".to_string());
        mapping.insert("З".to_string(), "Z".to_string());
        mapping.insert("И".to_string(), "I".to_string());
        mapping.insert("Й".to_string(), "I".to_string());
        mapping.insert("К".to_string(), "K".to_string());
        mapping.insert("Л".to_string(), "L".to_string());
        mapping.insert("М".to_string(), "M".to_string());
        mapping.insert("Н".to_string(), "N".to_string());
        mapping.insert("О".to_string(), "O".to_string());
        mapping.insert("П".to_string(), "P".to_string());
        mapping.insert("Р".to_string(), "R".to_string());
        mapping.insert("С".to_string(), "S".to_string());
        mapping.insert("Т".to_string(), "T".to_string());
        mapping.insert("У".to_string(), "U".to_string());
        mapping.insert("Ф".to_string(), "F".to_string());
        mapping.insert("Х".to_string(), "Kh".to_string());
        mapping.insert("Ц".to_string(), "Ts".to_string());
        mapping.insert("Ч".to_string(), "Ch".to_string());
        mapping.insert("Ш".to_string(), "Sh".to_string());
        mapping.insert("Щ".to_string(), "Shch".to_string());
        mapping.insert("Ъ".to_string(), "Ie".to_string());
        mapping.insert("Ы".to_string(), "Y".to_string());
        mapping.insert("Ь".to_string(), "".to_string());
        mapping.insert("Э".to_string(), "E".to_string());
        mapping.insert("Ю".to_string(), "Iu".to_string());
        mapping.insert("Я".to_string(), "Ia".to_string());


        IcaoDoc9303 {
            name: "".to_string(),
            description: "".to_string(),
            url: "".to_string(),
            comments: vec![],
            mapping,
            next_mapping: None,
            prev_mapping: None,
            ending_mapping: None,
            samples: vec![]
        }
    }
}

impl Transliterator for IcaoDoc9303 {
    fn transliterate(&self, input: &str) -> &str {
        let mut output = String::from("");
        let transliteration = input
            .split_word_bounds()
            .map(|word| {
                return &self.translate_word(word)
            })
            .collect::<Vec<&String>>()
            .join("");

        return &output;
    }

}

#[cfg(test)]
mod tests {
    use crate::Transliterator;

    #[test]
    fn first() {
        let icao = crate::schemas::IcaoDoc9303::default();
        let o = icao.transliterate("Юлия Щеглова");
        assert_eq!(o, "Iuliia Shcheglova");
    }
}
