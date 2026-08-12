use crate::entity::{ast::AST, psq::PSQ};
use crate::shared::parse_error::ParseError;
use crate::usecase::{or::fold_or, psq::apply_quantifier};
use std::mem::take;

/// 特殊文字エスケープ
fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        '\\' | '(' | ')' | '|' | '+' | '*' | '?' => Ok(AST::Char(c)),
        _ => {
            let err = ParseError::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

/// 正規表現を抽象構文木に変換
pub fn parse(expr: &str) -> Result<AST, Box<ParseError>> {
    // 内部状態を表現するための型
    // Char: 文字列処理中
    // Escape: エスケープシーケンス処理中
    enum ParseState {
        Char,
        Escape,
    }

    let mut seq = Vec::new();
    let mut seq_or = Vec::new();
    let mut stack = Vec::new();
    let mut state = ParseState::Char;

    for (i, c) in expr.chars().enumerate() {
        match &state {
            ParseState::Char => {
                match c {
                    '+' => {
                        let new_ast = apply_quantifier(seq.pop(), PSQ::Plus, i)?;
                        seq.push(new_ast);
                    }
                    '*' => {
                        let new_ast = apply_quantifier(seq.pop(), PSQ::Star, i)?;
                        seq.push(new_ast);
                    }
                    '?' => {
                        let new_ast = apply_quantifier(seq.pop(), PSQ::Question, i)?;
                        seq.push(new_ast);
                    }
                    '(' => {
                        let _prev = take(&mut seq);
                        let _prev_or = take(&mut seq_or);
                        stack.push((_prev, _prev_or));
                    }
                    ')' => {
                        if let Some((mut prev, prev_or)) = stack.pop() {
                            // "()"のように、式が空の場合はpushしない
                            if !seq.is_empty() {
                                seq_or.push(AST::Seq(seq));
                            }

                            // Orを生成
                            if let Some(ast) = fold_or(seq_or) {
                                prev.push(ast);
                            }

                            seq = prev;
                            seq_or = prev_or;
                        } else {
                            return Err(Box::new(ParseError::InvalidRightParen(i)));
                        }
                    }
                    '|' => {
                        if seq.is_empty() {
                            // "||", "(|abc)"などと、式が空の場合はエラー
                            return Err(Box::new(ParseError::NoPrev(i)));
                        } else {
                            let _prev = take(&mut seq);
                            seq_or.push(AST::Char(c));
                        }
                    }
                    '\\' => state = ParseState::Escape,
                    _ => seq.push(AST::Char(c)),
                };
            }

            ParseState::Escape => {
                let ast = parse_escape(i, c)?;
                seq.push(ast);
                state = ParseState::Char;
            }
        }
    }

    // 閉じカッコが足りない場合はエラー
    if !stack.is_empty() {
        return Err(Box::new(ParseError::NoRightParen));
    }

    // "()"のように、式が空の場合pushしない
    if !seq.is_empty() {
        seq_or.push(AST::Seq(seq));
    }

    // Orを生成し、成功した場合はそれを返す
    if let Some(ast) = fold_or(seq_or) {
        Ok(ast)
    } else {
        Err(Box::new(ParseError::Empty))
    }
}
