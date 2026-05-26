use crate::Schema;

/// Transliterate a string using the given schema.
pub fn transliterate<S: Schema>(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 2);
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if is_cyrillic(chars[i]) {
            let start = i;
            while i < chars.len() && is_cyrillic(chars[i]) {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            result.push_str(&transliterate_word::<S>(&word));
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

/// Check if a character is Cyrillic (covers basic Cyrillic + supplements for Uzbek).
fn is_cyrillic(c: char) -> bool {
    matches!(c, '\u{0400}'..='\u{04FF}' | '\u{0500}'..='\u{052F}')
}

/// Transliterate a single word (Cyrillic-only characters).
fn transliterate_word<S: Schema>(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    if chars.is_empty() {
        return String::new();
    }

    let (stem_len, ending_result) = if chars.len() >= 2 {
        let last2 = [
            to_lower(chars[chars.len() - 2]),
            to_lower(chars[chars.len() - 1]),
        ];
        match S::ending_mapping(last2) {
            Some(ending) => (chars.len() - 2, Some((ending, &chars[chars.len() - 2..]))),
            None => (chars.len(), None),
        }
    } else {
        (chars.len(), None)
    };

    let mut result = String::with_capacity(word.len() * 2);

    for i in 0..stem_len {
        let curr = chars[i];
        let is_upper = curr.is_uppercase();
        let curr_lower = to_lower(curr);

        let prev_lower = if i > 0 {
            Some(to_lower(chars[i - 1]))
        } else {
            None
        };
        let next_lower = chars.get(i + 1).and_then(|&c| {
            if i + 1 < stem_len {
                Some(to_lower(c))
            } else {
                None
            }
        });

        let translated = S::prev_mapping(prev_lower, curr_lower)
            .or_else(|| next_lower.and_then(|nl| S::next_mapping(curr_lower, nl)))
            .or_else(|| S::mapping(curr_lower));

        match translated {
            Some(s) => push_with_case(&mut result, s, is_upper),
            None => result.push(curr),
        }
    }

    if let Some((ending, ending_chars)) = ending_result {
        let all_upper = ending_chars.iter().all(|c| c.is_uppercase());
        if all_upper && !ending.is_empty() {
            for c in ending.chars() {
                for uc in c.to_uppercase() {
                    result.push(uc);
                }
            }
        } else {
            result.push_str(ending);
        }
    }

    result
}

fn to_lower(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

fn push_with_case(result: &mut String, s: &str, is_upper: bool) {
    if is_upper && !s.is_empty() {
        let mut c_iter = s.chars();
        if let Some(first) = c_iter.next() {
            for c in first.to_uppercase() {
                result.push(c);
            }
            result.push_str(c_iter.as_str());
        }
    } else {
        result.push_str(s);
    }
}
