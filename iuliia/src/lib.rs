/// Error returned when an unknown schema name is requested.
#[derive(Debug, Clone)]
pub struct UnknownSchemaError {
    name: String,
}

impl UnknownSchemaError {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl std::fmt::Display for UnknownSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown schema: {}", self.name)
    }
}

impl std::error::Error for UnknownSchemaError {}

/// A transliteration schema.
///
/// Each schema is a zero-sized type implementing this trait.
/// All mapping data is embedded in match statements — no runtime allocation needed.
pub trait Schema {
    /// Schema identifier (e.g., "wikipedia").
    const NAME: &'static str;

    /// Alternative names for this schema (e.g., "iso_9_1995" for gost_779).
    const ALIASES: &'static [&'static str] = &[];

    /// Base Cyrillic → Latin letter mapping.
    /// Input is always a lowercase Cyrillic character.
    /// Returns `None` if the character has no mapping in this schema.
    fn mapping(c: char) -> Option<&'static str>;

    /// Context-dependent mapping based on the previous letter.
    /// `prev` is `None` at word start, `Some(char)` otherwise (always lowercase).
    /// `curr` is always lowercase.
    ///
    /// Single-char keys in the JSON (e.g., `"е": "ye"`) mean "at word start" → `prev` is `None`.
    /// Two-char keys (e.g., `"ае": "ye"`) mean "after а" → `prev` is `Some('а')`.
    fn prev_mapping(prev: Option<char>, curr: char) -> Option<&'static str>;

    /// Context-dependent mapping based on the next letter.
    /// Both `curr` and `next` are always lowercase Cyrillic characters.
    fn next_mapping(curr: char, next: char) -> Option<&'static str>;

    /// Word ending mapping (last 2 characters of a word).
    /// Input is always 2 lowercase characters.
    fn ending_mapping(ending: [char; 2]) -> Option<&'static str>;

    /// Transliterate a string using this schema.
    fn transliterate(input: &str) -> String
    where
        Self: Sized,
    {
        engine::transliterate::<Self>(input)
    }
}

pub mod engine;
pub mod schemas;

// Re-export the transliterate-by-name function from generated schemas module
pub use schemas::transliterate;
