use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char, multispace0},
    combinator::{map, value},
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
    Component {
        name: String,
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
        parse_html_comment,
        parse_comment,
        parse_if,
        parse_for,
        parse_match,
        parse_expression,
        parse_call,
        parse_component_var,
        parse_element,
        parse_text,
    ))(input)
}

fn parse_comment(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("/*")(input)?;
    let (input, _) = take_until("*/")(input)?;
    let (input, _) = tag("*/")(input)?;
    Ok((input, Node::Text("".to_string())))
}

// Parse HTML comments <!-- ... --> and discard them
fn parse_html_comment(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("<!--")(input)?;
    let (input, _) = take_until("-->")(input)?;
    let (input, _) = tag("-->")(input)?;
    Ok((input, Node::Text("".to_string())))
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_' || c == ':'
}

fn parse_identifier(input: &str) -> IResult<&str, String> {
    let (input, name) = take_while1(is_identifier_char)(input)?;
    Ok((input, name.to_string()))
}

// Parse path like foo::bar::baz
fn parse_path(input: &str) -> IResult<&str, String> {
    let (input, parts) = many0(tuple((parse_identifier, tag("::"))))(input)?;

    let (input, last) = parse_identifier(input)?;

    let mut full_path = String::new();
    for (part, _) in parts {
        full_path.push_str(&part);
        full_path.push_str("::");
    }
    full_path.push_str(&last);

    Ok((input, full_path))
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

    // List of void/self-closing HTML elements that don't have closing tags
    let void_elements = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ];

    // Check if this is a void element
    let is_void = void_elements.contains(&name.as_str());

    if is_void {
        // Void elements have no children and no closing tag
        return Ok((
            input,
            Node::Element {
                name,
                attrs,
                children: vec![],
            },
        ));
    }

    // Special handling for script and style tags - skip their content
    let (input, children) = if name == "script" || name == "style" {
        // For script/style tags, consume everything until the closing tag
        // Build the closing tag pattern manually
        let remaining = input;
        let mut content_end = 0;
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

                if let Ok((_, _)) = parse_closing(slice) {
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

    let name_clone = name.clone();
    alt((
        // Static value: "foo"
        map(
            delimited(char('"'), take_until("\""), char('"')),
            move |s: &str| (name.clone(), AttributeValue::Static(s.to_string())),
        ),
        // Dynamic value: {expr}
        map(
            delimited(char('{'), take_balanced('{', '}'), char('}')),
            move |s: &str| (name_clone.clone(), AttributeValue::Dynamic(s.to_string())),
        ),
    ))(input)
}

// Helper to take content balanced by delimiters
fn take_balanced(open: char, close: char) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input: &str| {
        // eprintln!("take_balanced input: '{}'", input);
        let mut depth = 0;
        let mut len = 0;
        for c in input.chars() {
            if c == open {
                depth += 1;
            } else if c == close {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            len += c.len_utf8();
        }
        // eprintln!("take_balanced len: {}, content: '{}'", len, &input[..len]);
        let (input, content) = input.split_at(len);
        Ok((input, content))
    }
}

fn parse_expression(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('{')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = take_balanced('{', '}')(input)?;
    let (input, _) = char('}')(input)?;
    Ok((input, Node::Expression(expr.trim().to_string())))
}

fn parse_call(input: &str) -> IResult<&str, Node> {
    // panic!("parse_call called with: {}", input);
    // eprintln!("parse_call input: {}", input);
    let (input, _) = char('@')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = parse_path(input)?;
    // eprintln!("parse_call name: {}", name);
    let (input, _) = multispace0(input)?;

    // Manual parsing for debugging
    let (input, _) = match char::<&str, nom::error::Error<&str>>('(')(input) {
        Ok(res) => res,
        Err(e) => panic!("parse_call failed at '(': {:?}", e),
    };

    // Inline take_balanced logic for debugging
    let mut depth = 0;
    let mut len = 0;
    let open = '(';
    let close = ')';
    for c in input.chars() {
        // eprintln!("Char: '{}', depth: {}", c, depth);
        if c == open {
            depth += 1;
        } else if c == close {
            if depth == 0 {
                break;
            }
            depth -= 1;
        }
        len += c.len_utf8();
    }
    // eprintln!("Calculated len: {}", len);
    let (input, args) = input.split_at(len);

    let (input, _) = match char::<&str, nom::error::Error<&str>>(')')(input) {
        Ok(res) => res,
        Err(e) => panic!("parse_call failed at ')': {:?}, input: '{}'", e, input),
    };
    // eprintln!("parse_call args: {}", args);

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
            name,
            args: args.to_string(),
            _children: children,
        },
    ))
}

fn parse_component_var(input: &str) -> IResult<&str, Node> {
    panic!("parse_component_var called with: {}", input);
    let (input, _) = char('@')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = parse_path(input)?;

    Ok((input, Node::Component { name: name }))
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
        parse_comment,
        parse_if,
        parse_for,
        parse_match,
        parse_expression,
        parse_call,
        parse_component_var,
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
