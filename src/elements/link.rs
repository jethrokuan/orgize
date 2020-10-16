use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while, take_while_m_n},
    combinator::opt,
    sequence::delimited,
    IResult,
};

/// Link Object
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Link<'a> {
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub link_type: Option<Cow<'a, str>>,
    /// Link destination
    pub path: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub desc: Option<Cow<'a, str>>,
}

impl Link<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> Option<(&str, Link)> {
        parse_internal(input).ok()
    }

    pub(crate) fn parse_plain(input: &str) -> Option<(&str, Link)> {
        parse_plain_internal(input).ok()
    }

    pub fn into_owned(self) -> Link<'static> {
        Link {
            link_type: self.link_type.map(Into::into).map(Cow::Owned),
            path: self.path.into_owned().into(),
            desc: self.desc.map(Into::into).map(Cow::Owned),
        }
    }
}

#[inline]
fn parse_internal(input: &str) -> IResult<&str, Link, ()> {
    let (input, _) = (tag("[["))(input)?;
    let (input, link_type) = opt(delimited(
        tag(""),
        take_while(|c: char| c != '<' && c != '>' && c != '\n' && c != ']' && c != ':'),
        tag(":"),
    ))(input)?;
    let (input, path) = take_while(|c: char| c != '<' && c != '>' && c != '\n' && c != ']')(input)?;
    let (input, _) = tag("]")(input)?;
    let (input, desc) = opt(delimited(
        tag("["),
        take_while(|c: char| c != '[' && c != ']'),
        tag("]"),
    ))(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((
        input,
        Link {
            link_type: link_type.map(Into::into),
            path: path.into(),
            desc: desc.map(Into::into),
        },
    ))
}

#[inline]
fn parse_plain_internal(input: &str) -> IResult<&str, Link, ()> {
    let (input, link_type) = (take_while_m_n(1, 10, |c: char| {
        c != ' '
            && c != '\n'
            && c != ':'
            && c != '@'
            && c != ']'
            && c != '['
            && c != '.'
            && c != ','
    }))(input)?;
    let (input, _) = (tag(":"))(input)?;
    let (input, path) = (take_while(|c: char| {
        c != ' ' && c != '\n' && c != '@' && c != ']' && c != '[' && c != '.' && c != ','
    }))(input)?;
    Ok((
        input,
        Link {
            link_type: Some(link_type.into()),
            path: path.into(),
            desc: None,
        },
    ))
}

#[test]
fn parse() {
    assert_eq!(
        Link::parse("[[#id]]"),
        Some((
            "",
            Link {
                link_type: None,
                path: "#id".into(),
                desc: None
            }
        ))
    );
    assert_eq!(
        Link::parse("[[#id][desc]]"),
        Some((
            "",
            Link {
                link_type: None,
                path: "#id".into(),
                desc: Some("desc".into())
            }
        ))
    );
    assert_eq!(
        Link::parse("[[cite:foo][desc]]"),
        Some((
            "",
            Link {
                link_type: Some("cite".into()),
                path: "foo".into(),
                desc: Some("desc".into())
            }
        ))
    );
    assert!(Link::parse("[[#id][desc]").is_none());
}
