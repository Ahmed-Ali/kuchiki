use dom::HTMLDom;
use html5ever;
use std::borrow::Cow;
use tree::NodeRef;

/// Options for the HTML parser.
#[derive(Default)]
pub struct ParseOpts {
    /// Options for the HTML tokenizer.
    pub tokenizer: html5ever::tokenizer::TokenizerOpts,

    /// Options for the HTML tree builder.
    pub tree_builder: html5ever::tree_builder::TreeBuilderOpts,

    /// A callback for HTML parse errors (which are never fatal).
    pub on_parse_error: Option<Box<dyn FnMut(Cow<'static, str>)>>,
}

/// Parse an HTML document with html5ever and the default configuration.
pub fn parse_html() -> html5ever::Parser<HTMLDom> {
    parse_html_with_options(ParseOpts::default())
}

/// Parse an HTML document with html5ever with custom configuration.
pub fn parse_html_with_options(opts: ParseOpts) -> html5ever::Parser<HTMLDom> {
    let sink = HTMLDom {
        document_node: NodeRef::new_document(),
        on_parse_error: opts.on_parse_error,
    };
    let html5opts = html5ever::ParseOpts {
        tokenizer: opts.tokenizer,
        tree_builder: opts.tree_builder,
    };
    html5ever::parse_document(sink, html5opts)
}
