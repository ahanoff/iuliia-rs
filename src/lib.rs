pub mod iuliia {
    pub trait Transliterator {
        fn transliterate(input: String) -> String
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}