use crate::ast::AST;

pub fn fold_or(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 {
        let mut ast = seq_or.pop().unwrap();
        seq_or.reverse();
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast));
        }
        Some(ast)
    } else {
        seq_or.pop()
    }
}

mod tests {
    use super::*;
    use crate::ast::AST;

    #[test]
    fn test_fold_or_emply() {
        // 要素が空の場合
        let input: Vec<AST> = vec![];
        assert_eq!(fold_or(input), None);
    }

    #[test]
    fn test_fold_or_single_element() {
        // 要素が1個の場合
        let input: Vec<AST> = vec![AST::Char('a')];
        let expected = Some(AST::Char('a'));

        assert_eq!(fold_or(input), expected);
    }

    #[test]
    fn test_fold_or_two_elements() {
        // 要素が2個の場合
        let input: Vec<AST> = vec![AST::Char('a'), AST::Char('b')];
        let expected = Some(AST::Or(Box::new(AST::Char('a')), Box::new(AST::Char('b'))));

        assert_eq!(fold_or(input), expected);
    }

    #[test]
    fn test_fold_or_three_elements_right_associative() {
        // 要素が3個のケース（右結合になるかを検証）
        let input = vec![AST::Char('a'), AST::Char('b'), AST::Char('c')];

        // 期待されるツリー構造: a | (b | c)
        let expected = Some(AST::Or(
            Box::new(AST::Char('a')),
            Box::new(AST::Or(Box::new(AST::Char('b')), Box::new(AST::Char('c')))),
        ));

        assert_eq!(fold_or(input), expected);
    }
}
