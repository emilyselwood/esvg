//! Tools for reading in svg files
use crate::error::Error;
use crate::{Element, Node};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::str;

pub fn parse_string(input: String) -> Result<Element, Error> {
    let mut reader = Reader::from_str(input.as_str());
    reader.trim_text(true);

    let mut stack: Vec<Element> = vec![];
    let mut result: Option<Element> = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                stack.push(to_element(e)?);
            }
            Ok(Event::Empty(ref e)) => {
                let mut parent = stack.pop().unwrap();
                parent.add(&to_element(e)?);
                stack.push(parent);
            }
            Ok(Event::End(ref _e)) => {
                let current = stack.pop().unwrap();
                if !stack.is_empty() {
                    let mut parent = stack.pop().unwrap();
                    parent.add(&current);
                    stack.push(parent);
                } else {
                    result = Some(current);
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape()?.to_string();
                let mut parent = stack.pop().unwrap();
                parent.add_node(Node::Text(text));
                stack.push(parent);
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(Error::XMLError(e)),
        }
    }

    match result {
        Some(e) => Ok(e),
        None => Err(Error::EmptyDocument),
    }
}

fn to_element(e: &BytesStart) -> Result<Element, Error> {
    let mut element = Element::new(str::from_utf8(e.name().0)?);
    for attr in e.attributes() {
        match attr {
            Ok(a) => {
                _ = element.set(
                    str::from_utf8(a.key.0)?,
                    str::from_utf8(a.unescape_value()?.as_bytes())?,
                )
            }
            Err(e) => return Err(Error::XMLAttrError(e)),
        }
    }

    Ok(element)
}

#[cfg(test)]
mod tests {

    use super::parse_string;

    #[test]
    fn round_trip() {
        let input = "<foo c=\"green\" s=\"12.5, 13.0\" x=\"-10\" y=\"10px\">\n\
        \t<bar />\n\
        \t<frob />\n\
        \t<wizzz a=\"b\">\n\
        \t\t<bar />\n\
        \t</wizzz>\n\
        </foo>\n\
        "
        .to_string();

        let result = parse_string(input).unwrap();

        assert_eq!(
            result.to_pretty_string(),
            "<foo c=\"green\" s=\"12.5, 13.0\" x=\"-10\" y=\"10px\">\n\
             \t<bar />\n\
             \t<frob />\n\
             \t<wizzz a=\"b\">\n\
            \t\t<bar />\n\
            \t</wizzz>\n\
             </foo>\n\
             "
        );
    }
}
