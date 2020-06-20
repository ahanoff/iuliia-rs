use crate::Transliterator;

pub struct Gost7034 {

}

impl Default for Gost7034 {
    fn default() -> Self {
        Gost7034 {}
    }
}

impl Transliterator for Gost7034 {
    fn transliterate(&self, _input: &str) -> String {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use crate::Transliterator;

    #[test]
    fn first() {
        let gost7034 = crate::schemas::Gost7034::default();
        let o = gost7034.transliterate("Юлия Щеглова");
        assert_eq!(o, "Iuliia Shcheglova");
    }
}
