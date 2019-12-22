use html5ever::serialize::TraversalScope::*;
use html5ever::serialize::{serialize, Serialize, SerializeOpts, Serializer, TraversalScope};
use html5ever::QualName;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;
use std::string::ToString;
use tree::{NodeData, NodeRef};
use xml5ever::serialize::serialize as xml_serialize;
use xml5ever::serialize::SerializeOpts as XmlSerializeOpts;

impl Serialize for NodeRef {
    fn serialize<S: Serializer>(
        &self,
        serializer: &mut S,
        traversal_scope: TraversalScope,
    ) -> Result<()> {
        match (traversal_scope, self.data()) {
            (ref scope, &NodeData::Element(ref element)) => {
                if *scope == IncludeNode {
                    let attrs = element.attributes.borrow();

                    // Unfortunately we need to allocate something to hold these &'a QualName
                    let attrs = attrs
                        .map
                        .iter()
                        .map(|(name, attr)| {
                            (
                                QualName::new(
                                    attr.prefix.clone(),
                                    name.ns.clone(),
                                    name.local.clone(),
                                ),
                                &attr.value,
                            )
                        })
                        .collect::<Vec<_>>();

                    serializer.start_elem(
                        element.name.clone(),
                        attrs.iter().map(|&(ref name, value)| (name, &**value)),
                    )?
                }

                for child in self.children() {
                    Serialize::serialize(&child, serializer, IncludeNode)?
                }

                if *scope == IncludeNode {
                    serializer.end_elem(element.name.clone())?
                }
                Ok(())
            }

            (_, &NodeData::DocumentFragment) | (_, &NodeData::Document(_)) => {
                for child in self.children() {
                    Serialize::serialize(&child, serializer, IncludeNode)?
                }
                Ok(())
            }

            (ChildrenOnly(_), _) => Ok(()),

            (IncludeNode, &NodeData::Doctype(ref doctype)) => {
                serializer.write_doctype(&doctype.name)
            }
            (IncludeNode, &NodeData::Text(ref text)) => serializer.write_text(&text.borrow()),
            (IncludeNode, &NodeData::Comment(ref text)) => serializer.write_comment(&text.borrow()),
            (IncludeNode, &NodeData::ProcessingInstruction(ref contents)) => {
                let contents = contents.borrow();
                serializer.write_processing_instruction(&contents.0, &contents.1)
            }
        }
    }
}

impl ToString for NodeRef {
    #[inline]
    fn to_string(&self) -> String {
        let mut u8_vec = Vec::new();
        self.serialize(&mut u8_vec, true).unwrap();
        String::from_utf8(u8_vec).unwrap()
    }
}

impl NodeRef {
    /// Serialize this node and its descendants in HTML syntax to the given stream.
    #[inline]
    pub fn serialize<W: Write>(&self, writer: &mut W, include_self: bool) -> Result<()> {
        let scope = if include_self {
            IncludeNode
        } else {
            ChildrenOnly(None)
        };

        serialize(
            writer,
            self,
            SerializeOpts {
                traversal_scope: scope,
                ..Default::default()
            },
        )
    }

    /// Serialize this node and its descendants in XHTML syntax to the given stream.
    #[inline]
    pub fn xml_serialize<W: Write>(&self, writer: &mut W, include_self: bool) -> Result<()> {
        let scope = if include_self {
            IncludeNode
        } else {
            ChildrenOnly(None)
        };

        xml_serialize(
            writer,
            self,
            XmlSerializeOpts {
                traversal_scope: scope,
            },
        )
    }

    /// Serialize this node and its descendants in HTML syntax to a new file at the given path.
    #[inline]
    pub fn serialize_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(&path)?;
        self.serialize(&mut file, true)
    }

    /// Returns the outer HTML of the node
    pub fn html(&self) -> String {
        self.to_string()
    }

    /// Returns the inner HTML of the node
    pub fn inner_html(&self) -> String {
        let mut u8_vec = Vec::new();
        self.serialize(&mut u8_vec, false).unwrap();
        String::from_utf8(u8_vec).unwrap()
    }

    /// Returns the outer XHMTL of the node
    pub fn xhtml(&self) -> String {
        let mut u8_vec = Vec::new();
        self.xml_serialize(&mut u8_vec, true).unwrap();
        String::from_utf8(u8_vec).unwrap()
    }

    /// Returns the inner XHMTL of the node
    pub fn inner_xhtml(&self) -> String {
        let mut u8_vec = Vec::new();
        self.xml_serialize(&mut u8_vec, false).unwrap();
        String::from_utf8(u8_vec).unwrap()
    }
}
