use memchr::memchr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct FnDef;

impl FnDef {
    pub fn parse(src: &str) -> Option<(&str, &str, usize)> {
        debug_assert!(src.starts_with("[fn:"));

        let label = memchr(b']', src.as_bytes()).filter(|&i| {
            i != 4
                && src.as_bytes()[4..i]
                    .iter()
                    .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        })?;

        let end = eol!(src);

        Some((&src[4..label], &src[label + 1..end], end))
    }
}

#[test]
fn parse() {
    assert_eq!(
        FnDef::parse("[fn:1] https://orgmode.org").unwrap(),
        (
            "1",
            " https://orgmode.org",
            "[fn:1] https://orgmode.org".len()
        )
    );
    assert_eq!(
        FnDef::parse("[fn:word_1] https://orgmode.org").unwrap(),
        (
            "word_1",
            " https://orgmode.org",
            "[fn:word_1] https://orgmode.org".len()
        )
    );
    assert_eq!(
        FnDef::parse("[fn:WORD-1] https://orgmode.org").unwrap(),
        (
            "WORD-1",
            " https://orgmode.org",
            "[fn:WORD-1] https://orgmode.org".len()
        )
    );
    assert_eq!(
        FnDef::parse("[fn:WORD]").unwrap(),
        ("WORD", "", "[fn:WORD]".len())
    );
    assert!(FnDef::parse("[fn:] https://orgmode.org").is_none());
    assert!(FnDef::parse("[fn:wor d] https://orgmode.org").is_none());
    assert!(FnDef::parse("[fn:WORD https://orgmode.org").is_none());
}
