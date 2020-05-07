pub mod iuliia {
    pub trait Transliterator {
        fn transliterate(&self, input: String) -> String;
    }

    pub struct Wikipedia;
    impl Wikipedia {
        pub fn new() -> Self {
            Wikipedia {}
        }
    }

    impl Transliterator for Wikipedia {
        fn transliterate(&self, _input: String) -> String {
            return String::from("wiki")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iuliia::Transliterator;

    #[test]
    fn first() {
        let w = crate::iuliia::Wikipedia::new();
        let o = w.transliterate(String::from("Юлия, съешь ещё этих мягких французских булок из Йошкар-Олы, да выпей алтайского чаю"));
        assert_eq!(o, "Yuliya, syesh yeshchyo etikh myagkikh frantsuzskikh bulok iz Yoshkar-Oly, da vypey altayskogo chayu")
    }
}
