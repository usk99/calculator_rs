/// 字句解析の結果として得られるトークンの種類
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// 数値リテラル
    Number(f64),
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// 入力の終端
    Eof,
    /// '('
    LParen,
    /// ')'
    RParen,
    /// 関数
    Func(String),
    /// ','
    Comma,
}

/// 文字列をトークン列に分解する字句解析器
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    /// `pos` の文字を返す。`pos` は進めない。
    fn next_char(&self) -> Option<char> {
        if !self.eof()
            && let Some(c) = self.input[self.pos..].iter().next()
        {
            Some(*c)
        } else {
            None
        }
    }

    /// `pos` の文字を返し、`pos` を1文字分進める。
    fn consume_char(&mut self) -> Option<char> {
        if !self.eof()
            && let Some(c) = self.input[self.pos..].iter().next()
        {
            self.pos += 1;
            Some(*c)
        } else {
            None
        }
    }

    /// ファイルの終端
    fn eof(&self) -> bool {
        self.pos == self.input.len()
    }

    /// `test` が真を返す間、文字を消費して返す。
    fn consume_while<F: Fn(char) -> bool>(&mut self, test: F) -> String {
        let mut result = String::new();

        while !self.eof()
            && let Some(nc) = self.next_char()
            && test(nc)
        {
            if let Some(nc) = self.consume_char() {
                result.push(nc);
            }
        }
        result
    }

    /// 空白の読み飛ばし
    fn skip_whitespace(&mut self) {
        let _ = self.consume_while(char::is_whitespace);
    }

    /// 入力文字列全体を走査してトークン列を返す
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut result = Vec::new();

        while !self.eof() {
            // 空白スキップ
            self.skip_whitespace();

            if self.eof() {
                break;
            }

            match self.next_char() {
                Some('+') => {
                    result.push(Token::Plus);
                    self.consume_char();
                }
                Some('-') => {
                    result.push(Token::Minus);
                    self.consume_char();
                }
                Some('*') => {
                    result.push(Token::Star);
                    self.consume_char();
                }
                Some('/') => {
                    result.push(Token::Slash);
                    self.consume_char();
                }
                Some('(') => {
                    result.push(Token::LParen);
                    self.consume_char();
                }
                Some(')') => {
                    result.push(Token::RParen);
                    self.consume_char();
                }
                Some(',') => {
                    result.push(Token::Comma);
                    self.consume_char();
                }
                Some(c) if c.is_ascii_digit() => {
                    let num = self.consume_while(|c| c.is_numeric() || c == '.');
                    let n = num.parse::<f64>().map_err(|e| e.to_string())?;
                    result.push(Token::Number(n));
                }
                Some(c) if c.is_alphabetic() => {
                    let name = self.consume_while(|c| c.is_alphanumeric());
                    result.push(Token::Func(name));
                }
                Some(c) => {
                    return Err(c.to_string());
                }
                None => break,
            }
        }
        result.push(Token::Eof);
        Ok(result)
    }
}
