use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char, multispace0},
    combinator::value,
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Static(String),  // Static value: class="foo"
    Dynamic(String), // Dynamic value: class={expr}
}

#[derive(Debug, Clone)]
pub enum Node {
    Element {
        name: String,
        attrs: Vec<(String, AttributeValue)>,
        children: Vec<Node>,
    },
    Text(String),
    Expression(String),
    Call {
        name: String,
        args: String,
        _children: Vec<Node>,
    },
    If {
        condition: String,
        then_branch: Vec<Node>,
        else_branch: Option<Vec<Node>>,
    },
    For {
        pattern: String,
        iterator: String,
        body: Vec<Node>,
    },
}

pub fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    many0(preceded(multispace0, parse_node))(input)
}

pub fn parse_node(input: &str) -> IResult<&str, Node> {
    // Try structured nodes first, then fall back to text
    // This prevents text from consuming empty strings when input starts with special chars
    alt((
        parse_if,
        parse_for,
        parse_expression,
        parse_call,
        parse_element,
        parse_text,
    ))(input)
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_' || c == ':'
}

fn parse_identifier(input: &str) -> IResult<&str, String> {
    let (input, name) = take_while1(is_identifier_char)(input)?;
    Ok((input, name.to_string()))
}

pub fn parse_element(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('<')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;

    // Parse attributes
    let (input, attrs) = many0(preceded(multispace0, parse_attribute))(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = char('>')(input)?;

    // Special handling for script and style tags - skip their content
    let (input, children) = if name == "script" || name == "style" {
        // For script/style tags, consume everything until the closing tag
        // Build the closing tag pattern manually
        let remaining = input;
        let mut content_end = 0;

        // Search for </tagname>
        while content_end < remaining.len() {
            if remaining[content_end..].starts_with("</") {
                let after_slash = &remaining[content_end + 2..];
                if after_slash.starts_with(&name) {
                    // Check if followed by whitespace or >
                    let after_name = &after_slash[name.len()..];
                    if after_name.starts_with('>')
                        || after_name.starts_with(' ')
                        || after_name.starts_with('\t')
                        || after_name.starts_with('\n')
                    {
                        break;
                    }
                }
            }
            content_end += 1;
        }

        (&remaining[content_end..], vec![])
    } else {
        parse_nodes(input)?
    };

    let (input, _) = multispace0(input)?;
    let (input, _) = char('<')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('/')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(name.as_str())(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('>')(input)?;

    Ok((
        input,
        Node::Element {
            name: name,
            attrs,
            children,
        },
    ))
}

// Parse a single attribute: name="value" or name={expr}
fn parse_attribute(input: &str) -> IResult<&str, (String, AttributeValue)> {
    let (input, name) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace0(input)?;

    // Try dynamic attribute first: name={expr}
    let (input, value) = alt((parse_dynamic_attr_value, parse_static_attr_value))(input)?;

    Ok((input, (name, value)))
}

// Helper to take content until a balanced closing delimiter
fn take_balanced(open: char, close: char) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input: &str| {
        let mut depth = 0;
        let mut chars = input.char_indices();

        while let Some((i, c)) = chars.next() {
            if c == open {
                depth += 1;
            } else if c == close {
                if depth == 0 {
                    return Ok((&input[i..], &input[..i]));
                }
                depth -= 1;
            }
        }

        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeUntil,
        )))
    }
}

// Parse dynamic attribute value: {expr}
fn parse_dynamic_attr_value(input: &str) -> IResult<&str, AttributeValue> {
    let (input, expr) = delimited(char('{'), take_balanced('{', '}'), char('}'))(input)?;
    Ok((input, AttributeValue::Dynamic(expr.trim().to_string())))
}

// Parse static attribute value: "text" or 'text'
fn parse_static_attr_value(input: &str) -> IResult<&str, AttributeValue> {
    let (input, value) = alt((
        delimited(char('"'), take_until("\""), char('"')),
        delimited(char('\''), take_until("'"), char('\'')),
    ))(input)?;
    Ok((input, AttributeValue::Static(value.to_string())))
}

fn parse_expression(input: &str) -> IResult<&str, Node> {
    let (input, expr) = delimited(char('{'), take_balanced('{', '}'), char('}'))(input)?;
    Ok((input, Node::Expression(expr.trim().to_string())))
}

fn parse_call(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('@')(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, args) = delimited(char('('), take_balanced('(', ')'), char(')'))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, children) = alt((
        delimited(
            char('{'),
            preceded(multispace0, parse_nodes),
            preceded(multispace0, char('}')),
        ),
        value(vec![], multispace0),
    ))(input)?;

    Ok((
        input,
        Node::Call {
            name: name,
            args: args.to_string(),
            _children: children,
        },
    ))
}

fn parse_text(input: &str) -> IResult<&str, Node> {
    // Use take_while1 to ensure at least one character is consumed
    let (input, text) = take_while1(|c: char| c != '<' && c != '{' && c != '@')(input)?;
    Ok((input, Node::Text(text.to_string())))
}

fn parse_block_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    many0(parse_block_node)(input)
}

fn parse_block_node(input: &str) -> IResult<&str, Node> {
    // Try structured nodes first, then fall back to text (excluding '}')
    alt((
        parse_if,
        parse_for,
        parse_expression,
        parse_call,
        parse_element,
        parse_text_exclude_brace,
    ))(input)
}

fn parse_text_exclude_brace(input: &str) -> IResult<&str, Node> {
    let (input, text) = take_while1(|c| c != '<' && c != '{' && c != '@' && c != '}')(input)?;
    Ok((input, Node::Text(text.to_string())))
}

fn parse_if(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('@')(input)?;
    let (input, _) = tag("if")(input)?;
    let (input, _) = multispace0(input)?;
    // Parse condition until the opening brace
    let (input, condition) = take_until("{")(input)?;
    let (input, _) = char('{')(input)?;
    let (input, then_branch) = parse_block_nodes(input)?;
    let (input, _) = char('}')(input)?;

    // Check for else block
    let (input, else_branch) =
        match preceded(multispace0::<&str, nom::error::Error<&str>>, tag("else"))(input) {
            Ok((input, _)) => {
                let (input, _) = multispace0(input)?;
                let (input, _) = char('{')(input)?;
                let (input, else_nodes) = parse_block_nodes(input)?;
                let (input, _) = char('}')(input)?;
                (input, Some(else_nodes))
            }
            Err(_) => (input, None),
        };

    Ok((
        input,
        Node::If {
            condition: condition.trim().to_string(),
            then_branch,
            else_branch,
        },
    ))
}

fn parse_for(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('@')(input)?;
    let (input, _) = tag("for")(input)?;
    let (input, _) = multispace0(input)?;
    // Parse pattern (e.g. "item in items")
    let (input, pattern) = take_until("{")(input)?;
    let (input, _) = char('{')(input)?;
    let (input, body) = parse_block_nodes(input)?;
    let (input, _) = char('}')(input)?;

    Ok((
        input,
        Node::For {
            pattern: pattern.trim().to_string(),
            iterator: String::new(), // Iterator is now part of pattern in this simplified parser
            body,
        },
    ))
}
