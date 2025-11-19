use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{alphanumeric1, char, multispace0},
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
}

pub fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    many0(preceded(multispace0, parse_node))(input)
}

pub fn parse_node(input: &str) -> IResult<&str, Node> {
    // Try structured nodes first, then fall back to text
    // This prevents text from consuming empty strings when input starts with special chars
    alt((parse_expression, parse_call, parse_element, parse_text))(input)
}

fn parse_element(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('<')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;

    // Parse attributes
    let (input, attrs) = many0(preceded(multispace0, parse_attribute))(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = char('>')(input)?;

    let (input, children) = parse_nodes(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = char('<')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('/')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(name)(input)?;
    let (input, _) = multispace0(input)?;
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

// Parse a single attribute: name="value" or name={expr}
fn parse_attribute(input: &str) -> IResult<&str, (String, AttributeValue)> {
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace0(input)?;

    // Try dynamic attribute first: name={expr}
    let (input, value) = alt((parse_dynamic_attr_value, parse_static_attr_value))(input)?;

    Ok((input, (name.to_string(), value)))
}

// Parse dynamic attribute value: {expr}
fn parse_dynamic_attr_value(input: &str) -> IResult<&str, AttributeValue> {
    let (input, expr) = delimited(char('{'), take_until("}"), char('}'))(input)?;
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
    let (input, expr) = delimited(char('{'), take_until("}"), char('}'))(input)?;
    Ok((input, Node::Expression(expr.trim().to_string())))
}

fn parse_call(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('@')(input)?;
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, args) = delimited(char('('), take_until(")"), char(')'))(input)?;
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
            name: name.to_string(),
            args: args.to_string(),
            _children: children,
        },
    ))
}

fn parse_text(input: &str) -> IResult<&str, Node> {
    // Use take_while1 to ensure at least one character is consumed
    use nom::bytes::complete::take_while1;
    let (input, text) = take_while1(|c: char| c != '<' && c != '{' && c != '@' && c != '}')(input)?;
    Ok((input, Node::Text(text.to_string())))
}
