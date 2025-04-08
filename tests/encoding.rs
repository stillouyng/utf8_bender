#[cfg(test)]
mod tests {
    use utf8_bender::encode;
    use utf8_bender::encoder::EncodingError;

    #[test]
    fn test_ascii() {
        let result = encode("A");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "01000001 ");
    }

    #[test]
    fn test_cyrillic() {
        let result = encode("П");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "11010000 10011111 ");
    }

    #[test]
    fn test_special_symbols() {
        let first_result = encode("€");
        let second_result = encode("你");

        assert!(first_result.is_ok());
        assert!(second_result.is_ok());

        assert_eq!(first_result.unwrap(), "11100010 10000010 10101100 ");
        assert_eq!(second_result.unwrap(), "11100100 10111101 10100000 ");
    }

    #[test]
    fn test_emojis() {
        let result = encode("😊");

        assert!(result.is_ok());

        assert_eq!(
            encode("😊").unwrap(),
            "11110000 10011111 10011000 10001010 "
        );
    }

    #[test]
    fn test_template_filling() {
        let input = "Т";
        let expected = "11010000 10100010 ";
        let result = encode(input);

        assert!(result.is_ok());

        let encoded = result.unwrap();

        assert_eq!(encoded, expected);
        assert!(encoded.starts_with("110"));
        assert!(encoded.contains(" 10"));
    }

    #[test]
    fn test_invalid_unicode() {
        let invalid = "�";
        let result = encode(invalid);

        assert!(result.is_err());
        assert!(matches!(result, Err(EncodingError::InvalidCharacter('�'))));
    }
}
