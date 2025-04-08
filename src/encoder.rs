use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum EncodingError {
    InvalidCharacter(char),
}

pub fn encode(phrase: &str) -> Result<String, EncodingError> {
    if let Some(c) = phrase.chars().find(|c| *c == '�') {
        return Err(EncodingError::InvalidCharacter(c));
    }

    let mut response: String = String::new();

    let templates: HashMap<u8, &str> = [
        (1, "0xxxxxxx"),
        (2, "110xxxxx 10xxxxxx"),
        (3, "1110xxxx 10xxxxxx 10xxxxxx"),
        (4, "11110xxx 10xxxxxx 10xxxxxx 10xxxxxx"),
    ]
    .into();

    for char in phrase.chars() {
        let code = char as u32;

        let (template, total_bits) = match code {
            0..=127 => (templates[&1], 7),
            128..=2047 => (templates[&2], 11),
            2048..=65535 => (templates[&3], 16),
            65536..=1114111 => (templates[&4], 21),
            _ => continue,
        };
        let bits = format!("{:0width$b}", code, width = total_bits);
        let mut bits_iter = bits.chars();

        let encoded: String = template
            .chars()
            .map(|c| {
                if c == 'x' {
                    bits_iter.next().unwrap_or('0')
                } else {
                    c
                }
            })
            .collect();

        response.push_str(&encoded);
        response.push(' ');
    }
    Ok(response)
}
