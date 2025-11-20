use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char, multispace0},
    combinator::value,
    error::Error,
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Static(String),  // Static value: class="foo"
    Dynamic(String), // Dynamic value: class={expr}
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: String,
    pub body: Vec<Node>,
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
    Match {
        expr: String,
        arms: Vec<MatchArm>,
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
        parse_match,
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

fn parse_extended_identifier(input: &str) -> IResult<&str, String> {
    let (input, first) = parse_identifier(input)?;
    let (input, rest) = many0(tuple((
        multispace0,
        char('-'),
        multispace0,
        parse_identifier,
    )))(input)?;

    let mut name = first;
    for (_, _, _, part) in rest {
        name.push('-');
        name.push_str(&part);
    }
    Ok((input, name))
}

pub fn parse_element(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('<')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = parse_extended_identifier(input)?;
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
        let mut found = false;
        let mut found = false;

        // Search for </tagname> with optional spaces
        while content_end < remaining.len() {
            let slice = &remaining[content_end..];

            // Check if we are at the start of a closing tag
            if slice.starts_with("</")
                || (slice.starts_with("<") && slice[1..].trim_start().starts_with("/"))
            {
                // Potential match, try to parse it
                let mut parse_closing = tuple::<_, _, Error<&str>, _>((
                    char('<'),
                    multispace0,
                    char('/'),
                    multispace0,
                    tag(name.as_str()),
                    multispace0,
                    char('>'),
                ));

                if let Ok((rest, _)) = parse_closing(slice) {
                    found = true;
                    found = true;
                    break;
                }
            }

            content_end += 1;
        }

        if !found {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }

        let content = &remaining[..content_end];
        let input = &remaining[content_end..];

        (input, vec![Node::Text(content.to_string())])
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
    let (input, name) = parse_extended_identifier(input)?;
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
    let (input, _) = multispace0(input)?;
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
    let (input, _) = multispace0(input)?;
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
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("for")(input)?;
    let (input, _) = multispace0(input)?;
    // Parse pattern (e.g. "item in items") - take until { appears
    let (input, full_pattern) = take_while1(|c: char| c != '{')(input)?;
    let full_pattern = full_pattern.trim();

    let (pattern, iterator) = full_pattern
        .split_once(" in ")
        .map(|(p, i)| (p.trim(), i.trim()))
        .unwrap_or((full_pattern, ""));

    let (input, _) = multispace0(input)?; // Consume whitespace before {
    let (input, _) = char('{')(input)?;
    let (input, body) = parse_block_nodes(input)?;
    let (input, _) = char('}')(input)?;

    Ok((
        input,
        Node::For {
            pattern: pattern.to_string(),
            iterator: iterator.to_string(),
            body,
        },
    ))
}

fn parse_match(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('@')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("match")(input)?;
    let (input, _) = multispace0(input)?;

    // Parse the expression to match against
    let (input, expr) = take_while1(|c: char| c != '{')(input)?;
    let expr = expr.trim();

    let (input, _) = multispace0(input)?;
    let (input, _) = char('{')(input)?;
    let (input, _) = multispace0(input)?;

    // Parse match arms: pattern => { body }
    let (input, arms) = parse_match_arms(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = char('}')(input)?;

    Ok((
        input,
        Node::Match {
            expr: expr.to_string(),
            arms,
        },
    ))
}

fn parse_match_arms(input: &str) -> IResult<&str, Vec<MatchArm>> {
    let mut arms = Vec::new();
    let mut current_input = input;

    loop {
        // Skip whitespace
        let (input, _) = multispace0(current_input)?;

        // Check if we've reached the end of the match block
        if input.starts_with('}') {
            return Ok((input, arms));
        }

        // Parse pattern until =>
        let (input, pattern) = take_until("=>")(input)?;
        let (input, _) = tag("=>")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = char('{')(input)?;

        // Parse the body
        let (input, body) = parse_block_nodes(input)?;

        let (input, _) = char('}')(input)?;

        arms.push(MatchArm {
            pattern: pattern.trim().to_string(),
            body,
        });

        current_input = input;
    }
}
