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
        fn transliterate(&self, input: String) -> String {
            return String::from("wiki")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iuliia::Transliterator;

    #[test]
    fn test_add() {
        let w = crate::iuliia::Wikipedia::new();
        let o = w.transliterate(String::from("Юлия"));
        assert_eq!(o, "wiki")
    }
}
