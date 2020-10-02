extern crate unicode_segmentation;

pub mod schemas;

pub trait Transliterator {
    fn transliterate(&self, input: &str) -> String;
}