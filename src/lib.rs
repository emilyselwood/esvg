#![doc = include_str!("../README.md")]

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

pub mod convert;
pub mod error;
pub mod page;
pub mod path;
pub mod read;
pub mod shapes;
pub mod text;
pub mod value;

use crate::error::Error;
use crate::page::Page;

/// Create a new document with the width, height, and view box setup for the provided page.
pub fn create_document(paper: &Page) -> Element {
    let mut el = Element::new("svg");
    el.set("xmlns", "http://www.w3.org/2000/svg");
    el.set("xmlns:xlink", "http://www.w3.org/1999/xlink");
    el.set(
        "viewBox",
        format!("0, 0, {}, {}", paper.width, paper.height),
    );
    el.set(
        "width",
        format!("{}mm", convert::pixels_to_mm(paper.width, paper.dpi)),
    );
    el.set(
        "height",
        format!("{}mm", convert::pixels_to_mm(paper.height, paper.dpi)),
    );

    el
}

/// Write the provided document to a file at the given path.
pub fn save(path: &str, doc: &Element) -> Result<(), Error> {
    let mut f = File::create(path)?;
    write!(f, "{}", doc.to_pretty_string())?;
    Ok(())
}

/// Read an svg from the given path.
pub fn read(path: &str) -> Result<Element, Error> {
    let mut f = File::open(path)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    read::parse_string(buffer)
}

/// Defines a node in the xml tree
#[derive(Debug, Clone)]
pub enum Node {
    Text(String),
    Element(Element),
    Comment(String), // TODO: cdata blocks
}

impl fmt::Display for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Text(s) => write!(formatter, "{s}"),
            Node::Element(e) => write!(formatter, "{e}"),
            Node::Comment(c) => write!(formatter, "<!-- {c} -->"),
        }
    }
}

/// An svg xml tag
#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub children: Vec<Node>,
    attributes: HashMap<String, value::Value>,
}

impl Element {
    /// Create a new Element with the provided tag name
    ///
    /// ```
    /// let el = esvg::Element::new("g");
    /// assert_eq!(el.to_pretty_string(), "<g />\n".to_string());
    /// ```
    pub fn new(name: &str) -> Self {
        Element {
            name: name.to_string(),
            children: vec![],
            attributes: HashMap::new(),
        }
    }

    /// Short hand for creating a group element
    ///
    /// ```
    /// let el = esvg::Element::new("g");
    /// let group = esvg::Element::group();
    ///
    /// assert_eq!(el.to_pretty_string(), group.to_pretty_string());
    /// ```
    pub fn group() -> Self {
        Element::new("g")
    }

    // TODO: add more short hand methods like group here

    /// An element is a node of type element.
    pub fn as_node(&self) -> Node {
        Node::Element(self.clone())
    }

    /// Add a child element to this element
    pub fn add(&mut self, child: &Element) {
        self.children.push(Node::Element(child.clone()));
    }

    /// Add a child node to this element
    pub fn add_node(&mut self, child: Node) {
        self.children.push(child);
    }

    /// Set an attribute on this element
    pub fn set<K, T>(&mut self, key: K, value: T) -> &mut Self
    where
        K: Into<String>,
        T: Into<value::Value>,
    {
        self.attributes.insert(key.into(), value.into());

        self
    }

    /// Get the value of an attribute on this element
    pub fn get<K>(&self, key: K) -> Option<String>
    where
        K: Into<String>,
    {
        self.attributes.get(&key.into()).map(|s| s.to_string_bare())
    }

    pub fn add_style<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<value::Value>,
    {
        let new_style = match self.attributes.get("style") {
            Some(existing) => {
                format!(
                    "{};{}:{}",
                    existing.to_string_bare(),
                    key.into(),
                    value.into().to_string_bare()
                )
            }
            None => format!("{}:{}", key.into(), value.into().to_string_bare()),
        };

        self.attributes.insert("style".into(), new_style.into());

        self
    }

    pub fn style_map(&self) -> Result<HashMap<String, String>, Error> {
        let mut result = HashMap::new();

        if let Some(v) = self.attributes.get("style") {
            for e in v.to_string_bare().split(';') {
                if let Some((key, value)) = e.split_once(':') {
                    result.insert(key.to_string(), value.to_string());
                } else {
                    return Err(Error::MalformedStyle);
                }
            }
        }

        Ok(result)
    }

    /// Create a copy of this element with out its children
    pub fn shallow_clone(&self) -> Element {
        let mut result = Element::new(self.name.as_str());
        for (k, v) in &self.attributes {
            result.set(k.clone(), v.clone());
        }
        result
    }

    /// Create a nicely formatted string
    pub fn to_pretty_string(&self) -> String {
        let buff = if self.name == "svg" {
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.0//EN\" \"http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd\">\n".to_string()
        } else {
            "".to_string()
        };

        self.pretty_fmt_internal(buff, 0)
    }

    fn pretty_fmt_internal(&self, mut buff: String, depth: usize) -> String {
        buff = format!("{}{}<{}", buff, "\t".repeat(depth), self.name);

        let mut attributes = self.attributes.iter().collect::<Vec<_>>();
        attributes.sort_by_key(|pair| pair.0.as_str());
        for (k, v) in attributes {
            buff = format!("{buff} {k}={v}");
        }
        if !self.children.is_empty() {
            buff = format!("{buff}>\n");
            for child in self.children.iter() {
                buff = match child {
                    Node::Text(s) => format!("{buff}{s}"),
                    Node::Element(e) => e.pretty_fmt_internal(buff, depth + 1),
                    Node::Comment(c) => format!("{buff}<!-- {c} -->"),
                };
            }
            buff = format!("{}{}</{}>\n", buff, "\t".repeat(depth), self.name);
        } else {
            buff = format!("{buff} />\n");
        }

        buff
    }
}

impl fmt::Display for Element {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "<{}", self.name)?;
        let mut attributes = self.attributes.iter().collect::<Vec<_>>();
        attributes.sort_by_key(|pair| pair.0.as_str());
        for (k, v) in attributes {
            write!(formatter, " {k}={v}")?;
        }
        if !self.children.is_empty() {
            writeln!(formatter, ">")?;
            for child in self.children.iter() {
                writeln!(formatter, "{child}")?;
            }
            write!(formatter, "</{}>", self.name)
        } else {
            write!(formatter, " />")
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::Element;

    #[test]
    fn element_display() {
        let mut element = Element::new("foo");
        element.set("x", "-10");
        element.set("y", "10px");
        element.set("s", "12.5, 13.0");
        element.set("c", "green");
        element.add(&Element::new("bar"));

        assert_eq!(
            element.to_string(),
            "<foo c=\"green\" s=\"12.5, 13.0\" x=\"-10\" y=\"10px\">\n\
             <bar />\n\
             </foo>\
             "
        );
    }

    #[test]
    fn element_pretty_string() {
        let mut element = Element::new("foo");
        element
            .set("x", "-10")
            .set("y", "10px")
            .set("s", "12.5, 13.0")
            .set("c", "green");
        element.add(&Element::new("bar"));

        assert_eq!(
            element.to_pretty_string(),
            "<foo c=\"green\" s=\"12.5, 13.0\" x=\"-10\" y=\"10px\">\n\
             \t<bar />\n\
             </foo>\n\
             "
        );
    }

    #[test]
    fn element_add_style() {
        let mut element = Element::new("foo");
        assert_eq!(element.get("style"), None);
        element.add_style("stroke-width", 5.5);
        assert_eq!(element.get("style").unwrap(), "stroke-width:5.5");
        element.add_style("stroke", "#345623");
        assert_eq!(
            element.get("style").unwrap(),
            "stroke-width:5.5;stroke:#345623"
        );
    }

    #[test]
    fn element_style_map_happy() {
        let mut element = Element::new("foo");
        element
            .add_style("fish", "bob")
            .add_style("guppy", "slice")
            .add_style("key", "value");

        let result = element.style_map().unwrap();
        let mut expected = HashMap::new();
        expected.insert("guppy".to_string(), "slice".to_string());
        expected.insert("fish".to_string(), "bob".to_string());
        expected.insert("key".to_string(), "value".to_string());

        assert_eq!(result, expected);
    }

    #[test]
    fn element_style_map_unhappy() {
        let mut element = Element::new("foo");
        let result = element.style_map().unwrap();
        assert_eq!(format!("{:?}", result), "{}");

        element.set("style", "something broken");

        let result_broken = element.style_map();
        assert!(result_broken.is_err());
        // TODO: fix error types so that they can be compared.
        // For now we have to hope its the right error being returned
        // assert_eq!(result_broken.unwrap_err(), Error::MalformedStyle);
    }
}
