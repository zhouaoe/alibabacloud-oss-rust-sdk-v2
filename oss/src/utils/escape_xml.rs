use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufRead;
use std::rc::Rc;

use quick_xml::events::Event;
use quick_xml::Reader;

pub struct XmlDecoderLite<R: BufRead> {
    reader: Reader<R>,
    attribute_prefix: String,
}

#[derive(Default, Debug, Clone)]
pub struct XmlNode {
    children: Vec<XmlChild>,
    data: Vec<String>,
}

#[derive(Debug, Clone)]
struct XmlChild {
    k: String,
    children: Vec<Rc<RefCell<XmlNode>>>,
}

#[derive(Debug)]
struct Element {
    parent: Option<Box<Element>>,
    node: Rc<RefCell<XmlNode>>,
    label: String,
}

impl<R: BufRead> XmlDecoderLite<R> {
    pub fn new(inner: R) -> Self {
        let reader = Reader::from_reader(inner);
        XmlDecoderLite {
            reader,
            attribute_prefix: "+@".to_string(),
        }
    }

    pub fn decode(&mut self, root: Rc<RefCell<XmlNode>>) -> Result<(), Box<dyn std::error::Error>> {
        self.decode_xml(root)
    }

    fn decode_xml(&mut self, root: Rc<RefCell<XmlNode>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut elem = Box::new(Element {
            parent: None,
            node: root,
            label: String::new(),
        });

        let mut started = false;

        loop {
            let event = self.reader.read_event_into(&mut buf)?;
            // println!("\n{:#?}", event);
            match event {
                Event::Start(ref e) => {
                    // println!("Start: {:#?}", e);
                    elem = Box::new(Element {
                        parent: Some(elem),
                        node: Rc::new(RefCell::new(XmlNode::default())),
                        label: String::from_utf8_lossy(e.name().as_ref()).to_string(),
                    });

                    for attribute in e.attributes() {
                        let attr = attribute?;
                        let key = format!(
                            "{}{}",
                            self.attribute_prefix,
                            String::from_utf8_lossy(attr.key.as_ref())
                        );
                        let value = String::from_utf8_lossy(&attr.value).to_string();
                        // new_node
                        elem.node
                            .borrow_mut()
                            .add_child(&key, Rc::new(RefCell::new(XmlNode::new_with_data(value))));
                    }
                }
                Event::Text(e) => {
                    // println!("Text: {:#?}", e);
                    let trimmed = trim_non_graphic(std::str::from_utf8(e.unescape()?.as_bytes())?);
                    if !started && !trimmed.is_empty() {
                        return Err(format!(
                            "Invalid XML: Encountered chardata [{}] outside of XML node",
                            trimmed
                        )
                        .into());
                    }
                    if !trimmed.is_empty() {
                        elem.node.borrow_mut().data.push(trimmed);
                    }
                }
                Event::End(_e) => {
                    // println!("End: {:#?}", _e);
                    if let Some(parent) = elem.parent {
                        parent
                            .node
                            .borrow_mut()
                            .add_child(&elem.label, elem.node.clone());
                        elem = parent;
                    }
                }
                Event::Empty(e) => {
                    // println!("Empty: {:#?}", e);
                    // println! {"  before elem: {:#?}", elem}
                    let label = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let mut node = XmlNode::default();

                    for attribute in e.attributes() {
                        let attr = attribute?;
                        let key = format!(
                            "{}{}",
                            self.attribute_prefix,
                            String::from_utf8_lossy(attr.key.as_ref())
                        );
                        let value = String::from_utf8_lossy(&attr.value).to_string();
                        node.add_child(&key, Rc::new(RefCell::new(XmlNode::new_with_data(value))));
                    }
                    elem.node
                        .borrow_mut()
                        .add_child(&label, Rc::new(RefCell::new(node)));
                    // println! {"  after elem: {:#?}", elem}
                }
                Event::Eof => break,
                _ => {}
            }
            started = true;
            buf.clear();
        }
        Ok(())
    }
}

impl XmlNode {
    pub fn new_with_data(data: String) -> Self {
        XmlNode {
            children: vec![],
            data: vec![data],
        }
    }

    pub fn add_child(&mut self, s: &str, c: Rc<RefCell<XmlNode>>) {
        if let Some(child_entry) = self.children.iter_mut().find(|child| child.k == s) {
            child_entry.children.push(c);
        } else {
            self.children.push(XmlChild {
                k: s.to_string(),
                children: vec![c],
            });
        }
    }

    pub fn value(&self) -> Value {
        if !self.children.is_empty() {
            Value::Map(self.get_map())
        } else if !self.data.is_empty() {
            Value::Text(self.data[0].clone())
        } else {
            Value::Empty
        }
    }

    pub fn get_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        for child in &self.children {
            let label = &child.k;
            let children = &child.children;
            if children.len() > 1 {
                map.insert(
                    label.clone(),
                    Value::Array(children.iter().map(|x| x.borrow().value()).collect()),
                );
            } else {
                // println!("{:#?}", children[0].borrow().value());
                map.insert(label.clone(), children[0].borrow().value());
            }
        }
        map
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Value {
    #[default]
    Empty,
    Text(String),
    Map(HashMap<String, Value>),
    Array(Vec<Value>),
}

pub fn escape_xml(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '"' => result.push_str("&#34;"),
            '\'' => result.push_str("&#39;"),
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '\t' => result.push_str("&#x9;"),
            '\n' => result.push_str("&#xA;"),
            '\r' => result.push_str("&#xD;"),
            _ if !is_in_character_range(c) => {
                if (c as u32) < 0x20 {
                    result.push_str(&format!("&#x{:02X};", c as u32));
                } else {
                    result.push('\u{FFFD}');
                }
            }
            _ => result.push(c),
        }
    }
    result
}

pub fn trim_non_graphic(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|&c| {
            c == ' '
                || (!c.is_whitespace()
                    && !c.is_control()
                    && (c.is_ascii_graphic() || c.is_alphabetic()))
            // TODO no equivalent for is_graphic in Rust
        })
        .collect()
}

pub fn is_in_character_range(r: char) -> bool {
    r == '\u{0009}'
        || r == '\u{000A}'
        || r == '\u{000D}'
        || ('\u{0020}'..='\u{D7FF}').contains(&r)
        || ('\u{E000}'..='\u{FFFD}').contains(&r)
        || ('\u{10000}'..='\u{10FFFF}').contains(&r)
}

#[cfg(test)]
mod tests {
    // use std::any::Any;
    use std::io::Cursor;

    use super::*;

    const XML_DATA: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <root version="1.6" writer="jack">
        <node>
            <id>12345678</id>
            <uid>1234</uid>
        </node>
        <node>
            <id>22345678</id>
            <uid>2234</uid>
        </node>
        <tag key="tag1" value="value1"/>
        <tag key="tag2" value="value2"/>
        <foo>bar</foo>
        <node/>
        <empty></empty>
    </root>"#;

    #[test]
    fn test_xml_decode() {
        let mut decoder = XmlDecoderLite::new(Cursor::new(XML_DATA));
        let root = Rc::new(RefCell::new(XmlNode::default()));
        decoder.decode(root.clone()).unwrap();
        let value = root.borrow().get_map();
        // println!("{:#?}", value);

        assert!(value.contains_key("root"));
        if let Value::Map(root) = &value["root"] {
            assert_eq!(root.len(), 6);
            assert_eq!(root["+@version"], Value::Text("1.6".to_string()));
            assert_eq!(root["+@writer"], Value::Text("jack".to_string()));

            if let Value::Array(node_list) = &root["node"] {
                assert_eq!(node_list.len(), 3);

                if let Value::Map(first_node) = &node_list[0] {
                    assert_eq!(first_node.len(), 2);
                    assert_eq!(first_node["id"], Value::Text("12345678".to_string()));
                    assert_eq!(first_node["uid"], Value::Text("1234".to_string()));
                } else {
                    panic!();
                }

                if let Value::Map(second_node) = &node_list[1] {
                    assert_eq!(second_node.len(), 2);
                    assert_eq!(second_node["id"], Value::Text("22345678".to_string()));
                    assert_eq!(second_node["uid"], Value::Text("2234".to_string()));
                } else {
                    panic!();
                }

                assert_eq!(node_list[2], Value::Empty);
            } else {
                panic!();
            }

            if let Value::Array(tag_list) = &root["tag"] {
                assert_eq!(tag_list.len(), 2);

                if let Value::Map(first_tag) = &tag_list[0] {
                    assert_eq!(first_tag.len(), 2);
                    assert_eq!(first_tag["+@key"], Value::Text("tag1".to_string()));
                    assert_eq!(first_tag["+@value"], Value::Text("value1".to_string()));
                } else {
                    panic!();
                }

                if let Value::Map(second_tag) = &tag_list[1] {
                    assert_eq!(second_tag.len(), 2);
                    assert_eq!(second_tag["+@key"], Value::Text("tag2".to_string()));
                    assert_eq!(second_tag["+@value"], Value::Text("value2".to_string()));
                } else {
                    panic!();
                }
            } else {
                panic!();
            }

            assert_eq!(root["foo"], Value::Text("bar".to_string()));
            assert_eq!(root["empty"], Value::Empty);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_trim_non_graphic() {
        let test_cases = vec![
            ("foo", "foo"),
            (" foo", "foo"),
            ("foo ", "foo"),
            (" foo ", "foo"),
            ("   foo   ", "foo"),
            ("foo bar", "foo bar"),
            ("\n\tfoo\n\t", "foo"),
            ("\n\tfoo\n\tbar\n\t", "foobar"),
            ("", ""),
            ("\n", ""),
            ("\n\u{000B}", ""),
            ("ending with ä", "ending with ä"),
            ("ä and ä", "ä and ä"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(trim_non_graphic(input), expected);
        }
    }
}
