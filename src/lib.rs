extern crate unicode_segmentation;

mod schemas;

pub trait Transliterator {
    fn transliterate(&self, input: String) -> String;
}