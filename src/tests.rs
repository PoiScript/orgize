//! test utils

use nom::IResult;
use rowan::{ast::AstNode, SyntaxNode};

use crate::{
    syntax::{combinator::GreenElement, input::Input},
    ParseConfig,
};

pub fn to_ast<N: AstNode>(
    parser: impl Fn(Input) -> IResult<Input, GreenElement, ()>,
) -> impl Fn(&str) -> N {
    move |s: &str| {
        let input = Input {
            s,
            c: &ParseConfig::default(),
        };
        let element = parser(input).unwrap().1;
        let node = element.into_node().unwrap();
        let node = SyntaxNode::<N::Language>::new_root(node);
        AstNode::cast(node).unwrap()
    }
}
