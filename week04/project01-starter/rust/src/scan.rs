// scan.rs - an initial project01 scanner
//
// # Scanner EBNF (microsyntax)
//
// tokens ::= (token)*
// token  ::= intlit | symbol
// symbol ::= '+' | '-'
// intlit ::= digit (digit)*
// digit  ::= '0' | '1' | ... | '9'
//
// # Ignore
// whitespace ::= (' ' | '\t') (' ' | '\t')*

use std::fmt;
use std::process;

// ============================================================================
// Token types
// ============================================================================

/// Token type identifier for matching (without associated data)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    IntLit,
    Plus,
    Minus,
    Eot,
}

/// Token enum with associated data
#[derive(Debug, Clone)]
pub enum Token {
    IntLit(String), // Integer literal: "1", "22", "403"
    Plus,           // +
    Minus,          // -
    Eot,            // End of text
}

impl Token {
    /// Returns the string value of the token
    pub fn value(&self) -> &str {
        match self {
            Token::IntLit(s) => s,
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Eot => "",
        }
    }

    /// Returns the token name (for printing)
    pub fn name(&self) -> &str {
        match self {
            Token::IntLit(_) => "TK_INTLIT",
            Token::Plus => "TK_PLUS",
            Token::Minus => "TK_MINUS",
            Token::Eot => "TK_EOT",
        }
    }

    /// Returns the token type for matching
    pub fn token_type(&self) -> TokenType {
        match self {
            Token::IntLit(_) => TokenType::IntLit,
            Token::Plus => TokenType::Plus,
            Token::Minus => TokenType::Minus,
            Token::Eot => TokenType::Eot,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(\"{}\")", self.name(), self.value())
    }
}

// ============================================================================
// Scanner
// ============================================================================

/// Scanner for tokenizing input strings
struct Scanner {
    chars: Vec<char>,
    pos: usize,
}

impl Scanner {
    /// Create a new scanner for the given input
    fn new(input: &str) -> Self {
        Scanner {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    /// Check if we've reached the end of input
    fn at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }

    /// Get the current character (or '\0' if at end)
    fn current(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.chars[self.pos]
        }
    }

    /// Advance position and return the character we just passed
    fn advance(&mut self) -> char {
        let ch = self.current();
        self.pos += 1;
        ch
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while !self.at_end() && (self.current() == ' ' || self.current() == '\t') {
            self.advance();
        }
    }

    /// Scan an integer literal
    fn scan_intlit(&mut self) -> Token {
        let mut value = String::new();
        while !self.at_end() && self.current().is_ascii_digit() {
            value.push(self.advance());
        }
        Token::IntLit(value)
    }

    /// Scan a single token from the input
    fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.at_end() {
            return Token::Eot;
        }

        let ch = self.current();

        if ch.is_ascii_digit() {
            return self.scan_intlit();
        }

        // Single character tokens
        self.advance();
        match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            _ => {
                eprintln!("scan error: invalid char: {}", ch);
                process::exit(-1);
            }
        }
    }

    /// Scan all tokens from the input
    fn scan_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.scan_token();
            let is_eot = matches!(token, Token::Eot);
            tokens.push(token);
            if is_eot {
                break;
            }
        }
        tokens
    }
}

// ============================================================================
// Scan Table
// ============================================================================

/// Table of scanned tokens with current position tracking for parsing
pub struct ScanTable {
    tokens: Vec<Token>,
    cur: usize, // Current position for parsing
}

impl ScanTable {
    /// Create a new empty scan table
    pub fn new() -> Self {
        ScanTable {
            tokens: Vec::new(),
            cur: 0,
        }
    }

    /// Scan all tokens from the input string
    pub fn scan(&mut self, input: &str) {
        let mut scanner = Scanner::new(input);
        self.tokens = scanner.scan_all();
        self.cur = 0;
    }

    /// Print all tokens in the table
    pub fn print(&self) {
        for token in &self.tokens {
            println!("{}", token);
        }
    }

    /// Get the token at position cur + i
    /// Returns a reference to the token
    pub fn get(&self, i: isize) -> &Token {
        let index = (self.cur as isize + i) as usize;
        &self.tokens[index]
    }

    /// Accept the current token if it matches the expected type
    /// Returns true and advances cur if it matches, false otherwise
    pub fn accept(&mut self, expected: TokenType) -> bool {
        if self.cur >= self.tokens.len() {
            return false;
        }

        if self.tokens[self.cur].token_type() == expected {
            self.cur += 1;
            return true;
        }

        false
    }

    /// Accept any token (wildcard match) - always succeeds if not at end
    pub fn accept_any(&mut self) -> bool {
        if self.cur >= self.tokens.len() {
            return false;
        }
        self.cur += 1;
        true
    }
}
