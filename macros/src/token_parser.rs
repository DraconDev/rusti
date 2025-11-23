use proc_macro2::{Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::{Brace, Paren},
    Error, Ident, Result, Token,
};

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(Text),
    Expression(Expression),
    Comment(Comment),
    Doctype(Doctype),
    Fragment(Fragment),
    Block(Block),
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub children: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Static(String),
    Dynamic(TokenStream),
    None,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub content: TokenStream,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub content: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Doctype {
    pub content: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Fragment {
    pub children: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Block {
    If(IfBlock),
    For(ForBlock),
    Match(MatchBlock),
    Call(CallBlock),
    Component(ComponentBlock),
}

#[derive(Debug, Clone)]
pub struct IfBlock {
    pub condition: TokenStream,
    pub then_branch: Vec<Node>,
    pub else_branch: Option<Vec<Node>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ForBlock {
    pub pattern: TokenStream,
    pub iterator: TokenStream,
    pub body: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MatchBlock {
    pub expr: TokenStream,
    pub arms: Vec<MatchArm>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: TokenStream,
    pub body: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct CallBlock {
    pub name: syn::Path,
    pub args: TokenStream, // Named args or positional
    pub children: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ComponentBlock {
    pub name: syn::Path,
    pub span: Span,
}

// Parsing logic

pub fn parse_nodes(input: ParseStream) -> Result<Vec<Node>> {
    let mut nodes = Vec::new();
    while !input.is_empty() {
        if input.peek(Token![<]) {
            // Element, Comment, Doctype, Fragment
            if input.peek2(Token![!]) {
                // Comment or Doctype
                if input.peek3(Token![-]) {
                    nodes.push(Node::Comment(input.parse()?));
                } else {
                    nodes.push(Node::Doctype(input.parse()?));
                }
            } else if input.peek2(Token![/]) {
                // Closing tag - should be handled by parent element parser
                // If we see it here, it's an error (unmatched closing tag)
                // But we might be in a loop parsing children, so we stop.
                break;
            } else if input.peek2(Token![>]) {
                // Fragment < >
                nodes.push(Node::Fragment(input.parse()?));
            } else {
                // Element
                nodes.push(Node::Element(input.parse()?));
            }
        } else if input.peek(Token![@]) {
            // Block
            nodes.push(Node::Block(input.parse()?));
        } else if input.peek(Brace) {
            // Expression { ... }
            nodes.push(Node::Expression(input.parse()?));
        } else {
            // Text
            nodes.push(Node::Text(input.parse()?));
        }
    }
    Ok(nodes)
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let start_span = input.span();
        input.parse::<Token![<]>()?;
        let name = parse_html_name(input)?;
        eprintln!("Parsing element: {}", name);

        let mut attrs = Vec::new();
        while !input.peek(Token![>]) && !input.peek(Token![/]) {
            attrs.push(input.parse()?);
        }

        let mut children = Vec::new();
        if input.peek(Token![/]) {
            // Self-closing
            input.parse::<Token![/]>()?;
            input.parse::<Token![>]>()?;
        } else {
            input.parse::<Token![>]>()?;

            // Parse children
            if is_void_element(&name) {
                // Void element, no children, no closing tag
            } else {
                if name == "script" || name == "style" {
                    children.push(parse_raw_text_until_tag(input, &name)?);
                } else {
                    children = parse_nodes(input)?;
                }

                // Expect closing tag
                if input.peek(Token![<]) && input.peek2(Token![/]) {
                    eprintln!("Element::parse: Found closing tag sequence");
                    input.parse::<Token![<]>()?;
                    input.parse::<Token![/]>()?;
                    let closing_name = parse_html_name(input)?;
                    eprintln!(
                        "Found closing tag: </{}> (expected </{}>)",
                        closing_name, name
                    );
                    if closing_name != name {
                        return Err(Error::new(
                            input.span(),
                            format!(
                                "Mismatched closing tag: expected </{}>, found </{}>",
                                name, closing_name
                            ),
                        ));
                    }
                    input.parse::<Token![>]>()?;
                } else {
                    return Err(Error::new(
                        start_span,
                        format!("Unclosed element <{}>", name),
                    ));
                }
            }
        }

        Ok(Element {
            name,
            attrs,
            children,
            span: start_span,
        })
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = parse_html_name(input)?;
        let span = input.span();
        let value = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            if input.peek(Brace) {
                // {expr}
                let content;
                syn::braced!(content in input);
                AttributeValue::Dynamic(content.parse()?)
            } else if input.peek(syn::Lit) {
                let lit: syn::Lit = input.parse()?;
                match lit {
                    syn::Lit::Str(s) => AttributeValue::Static(s.value()),
                    _ => return Err(Error::new(span, "Expected string literal or expression")),
                }
            } else {
                return Err(Error::new(input.span(), "Expected attribute value"));
            }
        } else {
            AttributeValue::None
        };

        Ok(Attribute { name, value, span })
    }
}

impl Parse for Text {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let mut content = String::new();
        // Consume tokens until <, @, {, or }
        // Note: } is a delimiter for blocks, so we stop there too?
        // But text can contain } if it's not part of our structure?
        // No, } usually closes a block.

        while !input.is_empty()
            && !input.peek(Token![<])
            && !input.peek(Token![@])
            && !input.peek(Brace)
        {
            // If we hit a closing brace, it might be the end of a parent block.
            // But parse_nodes loop checks input.is_empty().
            // If parse_nodes is called inside a Brace, input will be empty at end of brace.
            // But if we are in a loop, we might see }.
            // Wait, parse_nodes doesn't check for }.
            // The caller of parse_nodes (e.g. Element parser) checks for </tag>.
            // What about Block parser?

            // We need to handle tokens.
            let tt: TokenTree = input.parse()?;
            content.push_str(&tt.to_string());
            // Add space? TokenStream loses spaces.
            // We can't easily restore them.
            // We'll assume single space separation for safety, or just concat.
            // concat is safer for "foo-bar", but "foo bar" becomes "foobar".
            // Ideally we check spans.
            // For now, let's append a space if it's not Punct?
            // Or just rely on to_string() which usually adds spaces.
        }

        if content.is_empty() {
            return Err(Error::new(span, "Unexpected empty text"));
        }

        Ok(Text { content, span })
    }
}

impl Parse for Expression {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let span = input.span();
        syn::braced!(content in input);
        Ok(Expression {
            content: content.parse()?,
            span,
        })
    }
}

impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![@]>()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![if]) {
            Ok(Block::If(input.parse()?))
        } else if lookahead.peek(Token![for]) {
            Ok(Block::For(input.parse()?))
        } else if lookahead.peek(Token![match]) {
            Ok(Block::Match(input.parse()?))
        } else {
            // Component or Call
            // Check if it's a path
            let path: syn::Path = input.parse()?;
            eprintln!(
                "Block::parse: Parsed path: {:?}",
                path.to_token_stream().to_string()
            );
            if input.peek(Paren) {
                // Call @foo(...)
                let content;
                let _ = syn::parenthesized!(content in input);
                let args = content.parse()?;

                // Optional children { ... }
                let children = if input.peek(Brace) {
                    let child_content;
                    syn::braced!(child_content in input);
                    parse_nodes(&child_content)?
                } else {
                    Vec::new()
                };

                let span = path.span();
                Ok(Block::Call(CallBlock {
                    name: path,
                    args,
                    children,
                    span,
                }))
            } else {
                // Component variable @foo
                let span = path.span();
                Ok(Block::Component(ComponentBlock { name: path, span }))
            }
        }
    }
}

// Helpers

fn parse_html_name(input: ParseStream) -> Result<String> {
    let mut name = String::new();
    if input.peek(Ident)
        || input.peek(Token![type])
        || input.peek(Token![for])
        || input.peek(Token![match])
        || input.peek(Token![async])
    {
        // We can't use Ident::parse_any easily because it's an extension trait method on Ident type?
        // No, it's `Ident::parse_any(input)`.
        // Wait, `Ident::parse_any` is a function in `ext::IdentExt` trait?
        // `fn parse_any(input: ParseStream) -> Result<Self>`
        // Yes.
        let ident = Ident::parse_any(input)?;
        name.push_str(&ident.to_string());

        while input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            name.push('-');
            if input.peek(Ident) || input.peek(Token![type]) || input.peek(Token![for]) {
                let part = Ident::parse_any(input)?;
                name.push_str(&part.to_string());
            } else {
                // Allow numbers?
                if input.peek(syn::Lit) {
                    let lit: syn::Lit = input.parse()?;
                    name.push_str(&lit.to_token_stream().to_string());
                }
            }
        }
    } else {
        return Err(input.error("Expected identifier"));
    }
    Ok(name)
}

fn is_void_element(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

fn parse_raw_text_until_tag(input: ParseStream, tag_name: &str) -> Result<Node> {
    // Consume tokens until we see </tag_name>
    let span = input.span();
    let mut content = String::new();

    loop {
        if input.is_empty() {
            break;
        }

        if input.peek(Token![<]) && input.peek2(Token![/]) {
            let fork = input.fork();
            fork.parse::<Token![<]>()?;
            fork.parse::<Token![/]>()?;
            if let Ok(name) = parse_html_name(&fork) {
                if name == tag_name {
                    // Found closing tag
                    break;
                }
            }
        }

        let tt: TokenTree = input.parse()?;
        content.push_str(&tt.to_string());
    }

    Ok(Node::Text(Text { content, span }))
}

// Implementations for If, For, Match... (omitted for brevity, need to fill in)
impl Parse for IfBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![if]>()?;
        // Condition is everything until {
        let mut condition = TokenStream::new();
        while !input.peek(Brace) {
            let tt: TokenTree = input.parse()?;
            condition.extend(Some(tt));
        }

        let content;
        syn::braced!(content in input);
        let then_branch = parse_nodes(&content)?;

        let else_branch = if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            let content;
            syn::braced!(content in input);
            Some(parse_nodes(&content)?)
        } else {
            None
        };

        Ok(IfBlock {
            condition,
            then_branch,
            else_branch,
            span,
        })
    }
}

impl Parse for ForBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![for]>()?;

        // pattern in iterator
        // We need to find "in"
        let mut pre_in = TokenStream::new();
        while !input.peek(Token![in]) && !input.peek(Brace) {
            let tt: TokenTree = input.parse()?;
            pre_in.extend(Some(tt));
        }

        if input.peek(Token![in]) {
            input.parse::<Token![in]>()?;
        } else {
            return Err(Error::new(input.span(), "Expected 'in' in for loop"));
        }

        let mut iterator = TokenStream::new();
        while !input.peek(Brace) {
            let tt: TokenTree = input.parse()?;
            iterator.extend(Some(tt));
        }

        let content;
        syn::braced!(content in input);
        let body = parse_nodes(&content)?;

        Ok(ForBlock {
            pattern: pre_in,
            iterator,
            body,
            span,
        })
    }
}

impl Parse for MatchBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![match]>()?;

        let mut expr = TokenStream::new();
        while !input.peek(Brace) {
            let tt: TokenTree = input.parse()?;
            expr.extend(Some(tt));
        }

        let content;
        syn::braced!(content in input);

        // Parse arms
        let mut arms = Vec::new();
        while !content.is_empty() {
            // pattern => { body }
            let mut pattern = TokenStream::new();
            while !content.peek(Token![=>]) && !content.is_empty() {
                let tt: TokenTree = content.parse()?;
                pattern.extend(Some(tt));
            }

            if content.peek(Token![=>]) {
                content.parse::<Token![=>]>()?;
            } else {
                return Err(Error::new(content.span(), "Expected =>"));
            }

            let body_content;
            syn::braced!(body_content in content);
            let body = parse_nodes(&body_content)?;

            // Optional comma?
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }

            arms.push(MatchArm { pattern, body });
        }

        Ok(MatchBlock { expr, arms, span })
    }
}

impl Parse for Comment {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![<]>()?;
        input.parse::<Token![!]>()?;
        input.parse::<Token![-]>()?;
        input.parse::<Token![-]>()?;

        // Consume until -->
        let mut content = String::new();
        loop {
            if input.peek(Token![-]) && input.peek2(Token![-]) && input.peek3(Token![>]) {
                input.parse::<Token![-]>()?;
                input.parse::<Token![-]>()?;
                input.parse::<Token![>]>()?;
                break;
            }
            if input.is_empty() {
                return Err(Error::new(span, "Unclosed comment"));
            }
            let tt: TokenTree = input.parse()?;
            content.push_str(&tt.to_string());
        }

        Ok(Comment { content, span })
    }
}

impl Parse for Doctype {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![<]>()?;
        input.parse::<Token![!]>()?;
        // DOCTYPE html
        let mut content = String::new();
        while !input.peek(Token![>]) {
            let tt: TokenTree = input.parse()?;
            content.push_str(&tt.to_string());
            content.push(' ');
        }
        input.parse::<Token![>]>()?;
        Ok(Doctype { content, span })
    }
}

impl Parse for Fragment {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![<]>()?;
        input.parse::<Token![>]>()?;

        let children = parse_nodes(input)?;

        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        input.parse::<Token![>]>()?;

        Ok(Fragment { children, span })
    }
}
