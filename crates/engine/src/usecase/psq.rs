use crate::entity::{ast::AST, psq::PSQ};
use crate::shared::parse_error::ParseError;

pub fn apply_quantifier(prev: Option<AST>, ast_type: PSQ, pos: usize) -> Result<AST, ParseError> {
    let prev_ast = prev.ok_or(ParseError::NoPrev(pos))?;

    let new_ast = match ast_type {
        PSQ::Plus => AST::Plus(Box::new(prev_ast)),
        PSQ::Star => AST::Star(Box::new(prev_ast)),
        PSQ::Question => AST::Question(Box::new(prev_ast)),
    };

    Ok(new_ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_quantifier_plus() {
        let input = Some(AST::Char('a'));
        let expected = Ok(AST::Plus(Box::new(AST::Char('a'))));

        assert_eq!(apply_quantifier(input, PSQ::Plus, 0), expected);
    }
    #[test]
    fn test_apply_quantifier_star() {
        let input = Some(AST::Char('a'));
        let expected = Ok(AST::Star(Box::new(AST::Char('a'))));

        assert_eq!(apply_quantifier(input, PSQ::Star, 0), expected);
    }
    #[test]
    fn test_apply_quantifier_question() {
        let input = Some(AST::Char('a'));
        let expected = Ok(AST::Question(Box::new(AST::Char('a'))));

        assert_eq!(apply_quantifier(input, PSQ::Question, 0), expected);
    }

    #[test]
    fn test_apply_quantifier_error_no_prev() {
        let input = None;
        let expected = Err(ParseError::NoPrev(10));

        assert_eq!(apply_quantifier(input, PSQ::Plus, 10), expected);
    }
}
