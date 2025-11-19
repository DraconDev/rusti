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
    c.is_alphanumeric() || c == '_' || c == '-' || c == ':'
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    take_while1(is_identifier_char)(input)
}

fn parse_element(input: &str) -> IResult<&str, Node> {
    // Parse opening tag
    let (input, _) = char('<')(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;

    // Parse attributes
    let (input, attrs) = parse_attributes(input)?;

    // Check for self-closing tag
    let (input, is_self_closing) = alt((value(true, tag("/>")), value(false, char('>'))))(input)?;

    if is_self_closing {
        return Ok((
            input,
            Node::Element {
                name: name.to_string(),
                attrs,
                children: vec![],
            },
        ));
    }

    // Parse children
    let (input, children) = parse_block_nodes(input)?;

    // Parse closing tag
    let (input, _) = tag("</")(input)?;
    let (input, _) = tag(name)(input)?;
    let (input, _) = char('>')(input)?;

    Ok((
        input,
        Node::Element {
            name: name.to_string(),
            attrs,
            children,
        },
    ))
}

fn parse_attributes(input: &str) -> IResult<&str, Vec<(String, AttributeValue)>> {
    many0(parse_attribute)(input)
}

fn parse_attribute(input: &str) -> IResult<&str, (String, AttributeValue)> {
    let (input_before_attr, _) = multispace0(input)?;

    // If we encounter '>' or '/>', we're done with attributes
    if input_before_attr.starts_with('>') || input_before_attr.starts_with("/>") {
        // Return empty result by failing gracefully
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let (input, name) = parse_identifier(input_before_attr)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace0(input)?;

    // Try to parse dynamic value first (in curly braces)
    let (input, value) = alt((
        |i: &str| {
            let (i, _) = char('{')(i)?;
            let (i, expr) = take_balanced(i, '{', '}')?;
            let (i, _) = char('}')(i)?;
            Ok((i, AttributeValue::Dynamic(expr.to_string())))
        },
        |i: &str| {
            let (i, val) = delimited(char('"'), take_until("\""), char('"'))(i)?;
            Ok((i, AttributeValue::Static(val.to_string())))
        },
    ))(input)?;

    Ok((input, (name.to_string(), value)))
}

// Helper function to extract content between balanced braces
fn take_balanced(input: &str, open: char, close: char) -> IResult<&str, &str> {
    let mut depth = 1;
    let mut idx = 0;
    let chars: Vec<char> = input.chars().collect();

    while idx < chars.len() && depth > 0 {
        if chars[idx] == open {
            depth += 1;
        } else if chars[idx] == close {
            depth -= 1;
        }
        if depth > 0 {
            idx += 1;
        }
    }

    if depth == 0 {
        Ok((&input[idx..], &input[..idx]))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeUntil,
        )))
    }
}

fn parse_expression(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('{')(input)?;
    let (input, expr) = take_balanced(input, '{', '}')?;
    let (input, _) = char('}')(input)?;

    Ok((input, Node::Expression(expr.trim().to_string())))
}

fn parse_call(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('@')(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = char('(')(input)?;
    let (input, args) = take_until(")")(input)?;
    let (input, _) = char(')')(input)?;

    Ok((
        input,
        Node::Call {
            name: name.to_string(),
            args: args.to_string(),
            _children: vec![],
        },
    ))
}

pub fn parse_block_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    let mut nodes = Vec::new();
    let mut current_input = input;

    loop {
        let (input, _) = multispace0(current_input)?;

        // Check if we've reached a closing tag or brace
        if input.starts_with("</") || input.starts_with('}') {
            return Ok((input, nodes));
        }

        // Check if we've reached an else keyword (for if statements)
        if input.starts_with("else") {
            return Ok((input, nodes));
        }

        // If input is empty, return what we have
        if input.is_empty() {
            return Ok((input, nodes));
        }

        // Try to parse a node
        match parse_node(input) {
            Ok((remaining, node)) => {
                nodes.push(node);
                current_input = remaining;
            }
            Err(_) => {
                // If we can't parse a node, return what we have
                return Ok((input, nodes));
            }
        }
    }
}

fn parse_text(input: &str) -> IResult<&str, Node> {
    let end_chars = ['<', '{', '@', '}'];
    let text: String = input
        .chars()
        .take_while(|c| !end_chars.contains(c))
        .collect();

    if text.is_empty() {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeWhile1,
        )))
    } else {
        Ok((&input[text.len()..], Node::Text(text.to_string())))
    }
}

fn parse_if(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('@')(input)?;
    let (input, _) = tag("if")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, condition) = take_while1(|c: char| c != '{')(input)?;
    let condition = condition.trim();

    let (input, _) = multispace0(input)?;
    let (input, _) = char('{')(input)?;
    let (input, then_branch) = parse_block_nodes(input)?;
    let (input, _) = char('}')(input)?;

    // Try to parse else branch
    let (input, _) = multispace0(input)?;
    let (input, else_branch) = if input.starts_with("else") {
        let (input, _) = tag("else")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = char('{')(input)?;
        let (input, else_nodes) = parse_block_nodes(input)?;
        let (input, _) = char('}')(input)?;
        (input, Some(else_nodes))
    } else {
        (input, None)
    };

    Ok((
        input,
        Node::If {
            condition: condition.to_string(),
            then_branch,
            else_branch,
        },
    ))
}

fn parse_for(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('@')(input)?;
    let (input, _) = tag("for")(input)?;
    let (input, _) = multispace0(input)?;

    // Parse pattern more carefully - take until we hit whitespace followed by '{'
    // This handles cases like "@for item in items {" with variable whitespace
    let (input, pattern) = take_while1(|c: char| c != '{')(input)?;
    let pattern = pattern.trim(); // Trim whitespace from pattern

    let (input, _) = multispace0(input)?; // Consume any whitespace before '{'
    let (input, _) = char('{')(input)?;
    let (input, body) = parse_block_nodes(input)?;
    let (input, _) = char('}')(input)?;

    Ok((
        input,
        Node::For {
            pattern: pattern.to_string(),
            iterator: String::new(), // Iterator is now part of pattern in this simplified parser
            body,
        },
    ))
}

fn parse_match(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('@')(input)?;
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
