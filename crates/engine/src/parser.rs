use crate::{
    ast::AST,
    or::fold_or,
    psq::{PSQ, parse_plus_star_question},
};
use std::{
    error::Error,
    fmt::{self, Display},
    mem::take,
};

#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char),
    InvalidRightParen(usize),
    NoPrev(usize),
    NoRightParen,
    Empty,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, c) => {
                write!(f, "ParseError: invalid escape: pos = {pos}, char = `{c}`")
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "ParseError: invalid right parenthesis: pos = {pos}")
            }
            ParseError::NoPrev(pos) => {
                write!(f, "ParseError: no previous expression: pos = {pos}")
            }
            ParseError::NoRightParen => {
                write!(f, "ParseError: no right parenthesis")
            }
            ParseError::Empty => write!(f, "parseError: empty expression"),
        }
    }
}

impl Error for ParseError {}

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
                    '+' => parse_plus_star_question(&mut seq, PSQ::Plus, i)?,
                    '*' => parse_plus_star_question(&mut seq, PSQ::Star, i)?,
                    '?' => parse_plus_star_question(&mut seq, PSQ::Question, i)?,
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
