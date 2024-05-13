#[derive(Debug, PartialEq)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: i32
}

impl Token {
    pub fn new(token_type: TokenType, start: usize, current: usize, line: i32) -> Self
    {
        Self {
            token_type,
            start,
            length: current-start,
            line
        }
    }

    pub fn get_line(&self) -> i32 {
        self.line
    }

    pub fn set_line(&mut self, line: i32)
    {
        self.line = line;
    }
}

pub fn compile(code: String) -> Result<(), &'static str>
{
    let mut scanner = Scanner::new(&code);
    let line = -1;
    loop {
        let mut token = scanner.scan_token()?;
        if token.get_line() != line {
            println!("{}", token.get_line());
            token.set_line(line);
        } else {
            println!("   | ");
        }
        println!("Token: {:?}", token);

        if token.token_type == TokenType::Eof {
            break Ok(());
        }
    }
}


struct Scanner<'sourcecode> {
    code: &'sourcecode str,
    start: usize,
    current: usize,
    line: i32
}

impl<'sourcecode> Scanner<'sourcecode> {
    pub fn new(code: &'sourcecode String) -> Self
    {
        Self {
            code,
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_token(&mut self) -> Result<Token, &'static str> {
        self.start = self.current;
        if self.is_at_end() {
            return Ok(Token::new(
                TokenType::Eof,
                self.start,
                self.current,
                self.line
            ));
        }

        let token_type = match self.advance() {
            b'(' => TokenType::LeftParen,
            b')' => TokenType::RightParen,
            b'{' => TokenType::LeftBrace,
            b'}' => TokenType::RightBrace,
            b';' => TokenType::Semicolon,
            b',' => TokenType::Comma,
            b'.' => TokenType::Dot,
            b'+' => TokenType::Plus,
            b'-' => TokenType::Minus,
            b'/' => TokenType::Slash,
            b'*' => TokenType::Star,
            b'>' => if self.check_next_char(b'='){ TokenType::GreaterEqual } else { TokenType::Greater },
            b'<' => if self.check_next_char(b'='){ TokenType::LessEqual } else { TokenType::Less },
            b'=' => if self.check_next_char(b'='){ TokenType::EqualEqual } else { TokenType::Equal },
            b'!' => if self.check_next_char(b'='){ TokenType::BangEqual } else { TokenType::Bang },
            _ => return Err("Unexpected token"),
        };

        Ok(Token::new(
            token_type,
            self.start,
            self.current,
            self.line,
        ))
    }

    fn check_next_char(&mut self, char: u8) -> bool
    {
        if self.is_at_end() || self.peek() != char {
            return false;
        }
        self.current+=1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current == self.code.len()
    }

    fn advance(&mut self) -> u8
    {
        let c = self.code.as_bytes()[self.current];
        self.current+=1;
        c
    }

    fn peek(&self) -> u8
    {
        if self.is_at_end() {
            0
        } else {
            self.code.as_bytes()[self.current]
        }
    }

    fn peek_next(&self) -> u8
    {
        if self.current > self.code.len() - 2 {
            b'\0'
        } else {
            self.code.as_bytes()[self.current + 1]
        }
    }
}
