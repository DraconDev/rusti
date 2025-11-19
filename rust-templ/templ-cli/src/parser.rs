use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until, take_while1},
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    combinator::{map, recognize, value},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub struct TemplDefinition {
    pub name: String,
    pub args: String,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Element {
        name: String,
        attrs: Vec<(String, String)>,
        children: Vec<Node>,
    },
    Text(String),
    Expression(String),
    Call {
        name: String,
        args: String,
        children: Vec<Node>,
    },
}

pub fn parse_templ(input: &str) -> IResult<&str, TemplDefinition> {
    let (input, _) = tuple((tag("templ"), multispace1))(input)?;
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, args) = delimited(char('('), take_until(")"), char(')'))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, children) = delimited(
        char('{'),
        preceded(multispace0, parse_nodes),
        preceded(multispace0, char('}')),
    )(input)?;

    Ok((
        input,
        TemplDefinition {
            name: name.to_string(),
            args: args.to_string(),
            children,
        },
    ))
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    many0(preceded(multispace0, parse_node))(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    alt((
        parse_element,
        parse_expression,
        parse_call,
        parse_text,
    ))(input)
}

fn parse_element(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('<')(input)?;
    let (input, name) = alphanumeric1(input)?;
    // TODO: Parse attributes
    let (input, _) = char('>')(input)?;
    
    let (input, children) = parse_nodes(input)?;
    
    let (input, _) = tag("</")(input)?;
    let (input, _) = tag(name)(input)?;
    let (input, _) = char('>')(input)?;

    Ok((
        input,
        Node::Element {
            name: name.to_string(),
            attrs: vec![],
            children,
        },
    ))
}

fn parse_expression(input: &str) -> IResult<&str, Node> {
    let (input, expr) = delimited(char('{'), take_until("}"), char('}'))(input)?;
    Ok((input, Node::Expression(expr.trim().to_string())))
}

fn parse_call(input: &str) -> IResult<&str, Node> {
    let (input, _) = char('@')(input)?;
    let (input, name) = alphanumeric1(input)?; // TODO: Support paths like Component.Sub
    let (input, _) = multispace0(input)?;
    let (input, args) = delimited(char('('), take_until(")"), char(')'))(input)?;
    let (input, _) = multispace0(input)?;
    
    // Optional children block
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
            children,
        },
    ))
}

fn parse_text(input: &str) -> IResult<&str, Node> {
    let (input, text) = is_not("<{@}")(input)?;
    Ok((input, Node::Text(text.to_string())))
}
