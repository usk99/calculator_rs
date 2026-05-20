use crate::lexer::*;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    BinOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        if self.eof() {
            return None;
        }
        Some(&self.tokens[self.pos])
    }
    fn consume(&mut self) -> Option<Token> {
        if self.eof() {
            return None;
        }
        let token = Some(self.tokens[self.pos]);
        self.pos += 1;
        token
    }
    fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.consume() {
            Some(Token::Number(n)) => Ok(Expr::Number(n)),
            Some(t) => Err(format!("unexpected token: {:?}", t)),
            None => Err("unexpected end of input".to_string()),
        }
    }

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
