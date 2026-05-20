use crate::lexer::*;

/// 二項演算子の種類
#[derive(Debug, Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
}

/// 抽象構文木（AST）のノード
#[derive(Debug, Clone)]
pub enum Expr {
    /// 数値リテラル
    Number(f64),
    /// 二項演算（left op right）
    BinOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// 関数呼び出し（name(args...)）
    Call { name: String, args: Vec<Expr> },
}

/// トークン列を AST に変換する構文解析器
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// 現在位置のトークンを返す（消費しない）
    fn current(&self) -> Option<&Token> {
        if self.eof() {
            return None;
        }
        Some(&self.tokens[self.pos])
    }
    /// 現在位置のトークンを返して pos を進める
    fn consume(&mut self) -> Option<Token> {
        if self.eof() {
            return None;
        }
        let token = Some(self.tokens[self.pos].clone());
        self.pos += 1;
        token
    }
    /// トークン列の終端かどうか
    fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    /// 数値・カッコ・単項マイナス・関数呼び出しを解析する
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current() {
            Some(Token::LParen) => {
                self.consume();
                let expr = self.parse_expr()?;
                match self.consume() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err("expected ')'".to_string()),
                }
            }
            Some(Token::Minus) => {
                self.consume();
                let ope = self.parse_primary()?;
                Ok(Expr::BinOp {
                    op: Op::Minus,
                    left: Box::new(Expr::Number(0.0)),
                    right: Box::new(ope),
                })
            }
            Some(Token::Func(name)) => {
                let name = name.clone();
                self.consume(); // Func トークンを消費
                self.consume(); // `(` を消費
                let mut args = Vec::new();
                loop {
                    args.push(self.parse_expr()?);
                    match self.current() {
                        Some(Token::RParen) => {
                            self.consume();
                            break;
                        }
                        Some(Token::Comma) => {
                            self.consume();
                        }
                        _ => return Err("expected ',' or ')'".to_string()),
                    }
                }
                Ok(Expr::Call { name, args })
            }

            _ => match self.consume() {
                Some(Token::Number(n)) => Ok(Expr::Number(n)),
                Some(t) => Err(format!("unexpected token: {:?}", t)),
                None => Err("unexpected end of input".to_string()),
            },
        }
    }

    /// 加算・減算を解析する（最低優先順位）
    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        loop {
            let op = match self.current() {
                Some(Token::Plus) => Op::Plus,
                Some(Token::Minus) => Op::Minus,
                _ => break,
            };
            self.consume();

            let right = self.parse_term()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// 乗算・除算を解析する（加減算より高優先順位）
    pub fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        loop {
            let op = match self.current() {
                Some(Token::Star) => Op::Star,
                Some(Token::Slash) => Op::Slash,
                _ => break,
            };
            self.consume();

            let right = self.parse_primary()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }
}
