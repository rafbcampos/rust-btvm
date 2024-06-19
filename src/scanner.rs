use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // One or two character tokens.
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier(String),
    String(String),
    Number(f64),
    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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
    Error(String),
    Eof,
}

#[derive(Debug)]
pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    tokens: Vec<TokenType>,
    line: usize,
}

fn parse_number(mut scanner: Scanner<'_>) -> Scanner {
    if scanner.source.peek().is_none() || !scanner.source.peek().unwrap().is_numeric() {
        return scanner;
    }

    let mut number_string = String::new();

    while let Some(&ch) = scanner.source.peek() {
        if ch.is_numeric() || ch == '.' {
            number_string.push(ch);
            scanner.source.next();

            if ch == '\n' {
                scanner.line += 1;
                scanner.source.next();
            }

            if ch == '.' {
                if let Some(&next_ch) = scanner.source.peek() {
                    if !next_ch.is_numeric() {
                        scanner
                            .tokens
                            .push(TokenType::Error("Expected digit after '.'".to_string()));
                        return scanner;
                    }
                }
            }
        } else {
            break;
        }
    }

    if let Ok(number) = number_string.parse::<f64>() {
        scanner.tokens.push(TokenType::Number(number));
    } else {
        scanner
            .tokens
            .push(TokenType::Error("Failed to parse number".to_string()));
    }

    scanner
}

fn parse_string(mut scanner: Scanner<'_>) -> Scanner {
    if scanner.source.peek().is_none() || scanner.source.peek() != Some(&'"') {
        return scanner;
    }
    let mut string = String::new();
    scanner.source.next(); // Consume opening '"'
    while let Some(&ch) = scanner.source.peek() {
        if ch == '"' {
            scanner.source.next();
            break;
        }
        string.push(ch);
        scanner.source.next();
    }
    scanner.tokens.push(TokenType::String(string));
    scanner
}

fn parse_whitespace(mut scanner: Scanner<'_>) -> Scanner {
    while let Some(&ch) = scanner.source.peek() {
        match ch {
            ' ' | '\r' | '\t' => {
                scanner.source.next();
            }
            '\n' => {
                scanner.line += 1;
                scanner.source.next();
            }
            _ => break,
        }
    }
    scanner
}

fn parse_comment(mut scanner: Scanner<'_>) -> Scanner {
    if scanner.source.peek() == Some(&'/') {
        scanner.source.next();
        if scanner.source.peek() == Some(&'/') {
            while let Some(&ch) = scanner.source.peek() {
                if ch == '\n' {
                    scanner.line += 1;
                    scanner.source.next();
                    break;
                }
                scanner.source.next();
            }
        }
    }
    scanner
}

fn parse_identifier(mut scanner: Scanner<'_>) -> Scanner {
    match scanner.source.peek() {
        Some(&ch) if ch.is_alphabetic() || ch == '_' => {
            let mut identifier = String::new();
            while let Some(&ch) = scanner.source.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    identifier.push(ch);
                    scanner.source.next();
                } else {
                    break;
                }
            }

            // check against the reserved keywords
            match identifier.as_str() {
                "and" => scanner.tokens.push(TokenType::And),
                "class" => scanner.tokens.push(TokenType::Class),
                "else" => scanner.tokens.push(TokenType::Else),
                "false" => scanner.tokens.push(TokenType::False),
                "fun" => scanner.tokens.push(TokenType::Fun),
                "for" => scanner.tokens.push(TokenType::For),
                "if" => scanner.tokens.push(TokenType::If),
                "nil" => scanner.tokens.push(TokenType::Nil),
                "or" => scanner.tokens.push(TokenType::Or),
                "print" => scanner.tokens.push(TokenType::Print),
                "return" => scanner.tokens.push(TokenType::Return),
                "super" => scanner.tokens.push(TokenType::Super),
                "this" => scanner.tokens.push(TokenType::This),
                "true" => scanner.tokens.push(TokenType::True),
                "var" => scanner.tokens.push(TokenType::Var),
                "while" => scanner.tokens.push(TokenType::While),
                _ => scanner.tokens.push(TokenType::Identifier(identifier)),
            }
            scanner
        }
        _ => scanner,
    }
}

fn parse_pontuation(mut scanner: Scanner<'_>) -> Scanner {
    match scanner.source.peek() {
        Some(&ch) => match ch {
            '(' => {
                scanner.tokens.push(TokenType::LeftParen);
                scanner.source.next();
                scanner
            }
            ')' => {
                scanner.tokens.push(TokenType::RightParen);
                scanner.source.next();
                scanner
            }
            '{' => {
                scanner.tokens.push(TokenType::LeftBrace);
                scanner.source.next();
                scanner
            }
            '}' => {
                scanner.tokens.push(TokenType::RightBrace);
                scanner.source.next();
                scanner
            }
            ',' => {
                scanner.tokens.push(TokenType::Comma);
                scanner.source.next();
                scanner
            }
            '.' => {
                scanner.tokens.push(TokenType::Dot);
                scanner.source.next();
                scanner
            }
            '-' => {
                scanner.tokens.push(TokenType::Minus);
                scanner.source.next();
                scanner
            }
            '+' => {
                scanner.tokens.push(TokenType::Plus);
                scanner.source.next();
                scanner
            }
            '/' => {
                scanner.tokens.push(TokenType::Slash);
                scanner.source.next();
                scanner
            }
            ';' => {
                scanner.tokens.push(TokenType::Semicolon);
                scanner.source.next();
                scanner
            }
            '*' => {
                scanner.tokens.push(TokenType::Star);
                scanner.source.next();
                scanner
            }
            '!' => {
                scanner.source.next();
                match scanner.source.peek() {
                    Some(&'=') => {
                        scanner.tokens.push(TokenType::BangEqual);
                        scanner.source.next();
                    }
                    _ => scanner.tokens.push(TokenType::Bang),
                }
                scanner
            }
            '=' => {
                scanner.source.next();
                match scanner.source.peek() {
                    Some(&'=') => {
                        scanner.tokens.push(TokenType::EqualEqual);
                        scanner.source.next();
                    }
                    _ => scanner.tokens.push(TokenType::Equal),
                }
                scanner
            }
            '<' => {
                scanner.source.next();
                match scanner.source.peek() {
                    Some(&'=') => {
                        scanner.tokens.push(TokenType::LessEqual);
                        scanner.source.next();
                    }
                    _ => scanner.tokens.push(TokenType::Less),
                }
                scanner
            }
            '>' => {
                scanner.source.next();
                match scanner.source.peek() {
                    Some(&'=') => {
                        scanner.tokens.push(TokenType::GreaterEqual);
                        scanner.source.next();
                    }
                    _ => scanner.tokens.push(TokenType::Greater),
                }
                scanner
            }
            _ => scanner,
        },
        _ => scanner,
    }
}

fn pipe(scanner: Scanner<'_>, functions: Vec<fn(Scanner<'_>) -> Scanner<'_>>) -> Scanner<'_> {
    functions.into_iter().fold(scanner, |acc, f| f(acc))
}

pub fn scan(source: &str) -> Scanner {
    let functions = vec![
        parse_whitespace,
        parse_comment,
        parse_identifier,
        parse_number,
        parse_string,
        parse_pontuation,
    ];
    let mut scanner = Scanner {
        source: source.chars().peekable(),
        tokens: Vec::new(),
        line: 1,
    };
    while scanner.source.peek().is_some() {
        match scanner.tokens.last() {
            Some(TokenType::Error(_)) => return scanner,
            _ => scanner = pipe(scanner, functions.clone()),
        }
    }
    scanner.tokens.push(TokenType::Eof);
    scanner
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_token {
        ($scanner:expr, $index:expr, $token:expr) => {
            assert_eq!($scanner.tokens[$index], $token);
        };
    }
    #[test]
    fn test_scan() {
        let source = "( ) { } , . - + ; * ! != = == > >= < <= identifier \"string\" 123.45 and class else false fun for if nil or print return super this true var while\n // comment\n 1/3";
        let scanner = scan(source);
        println!("{:?}", scanner.tokens);
        assert_eq!(scanner.tokens.len(), 41);
        assert_token!(scanner, 0, TokenType::LeftParen);
        assert_token!(scanner, 1, TokenType::RightParen);
        assert_token!(scanner, 2, TokenType::LeftBrace);
        assert_token!(scanner, 3, TokenType::RightBrace);
        assert_token!(scanner, 4, TokenType::Comma);
        assert_token!(scanner, 5, TokenType::Dot);
        assert_token!(scanner, 6, TokenType::Minus);
        assert_token!(scanner, 7, TokenType::Plus);
        assert_token!(scanner, 8, TokenType::Semicolon);
        assert_token!(scanner, 9, TokenType::Star);
        assert_token!(scanner, 10, TokenType::Bang);
        assert_token!(scanner, 11, TokenType::BangEqual);
        assert_token!(scanner, 12, TokenType::Equal);
        assert_token!(scanner, 13, TokenType::EqualEqual);
        assert_token!(scanner, 14, TokenType::Greater);
        assert_token!(scanner, 15, TokenType::GreaterEqual);
        assert_token!(scanner, 16, TokenType::Less);
        assert_token!(scanner, 17, TokenType::LessEqual);
        assert_token!(scanner, 18, TokenType::Identifier("identifier".to_string()));
        assert_token!(scanner, 19, TokenType::String("string".to_string()));
        assert_token!(scanner, 20, TokenType::Number(123.45));
        assert_token!(scanner, 21, TokenType::And);
        assert_token!(scanner, 22, TokenType::Class);
        assert_token!(scanner, 23, TokenType::Else);
        assert_token!(scanner, 24, TokenType::False);
        assert_token!(scanner, 25, TokenType::Fun);
        assert_token!(scanner, 26, TokenType::For);
        assert_token!(scanner, 27, TokenType::If);
        assert_token!(scanner, 28, TokenType::Nil);
        assert_token!(scanner, 29, TokenType::Or);
        assert_token!(scanner, 30, TokenType::Print);
        assert_token!(scanner, 31, TokenType::Return);
        assert_token!(scanner, 32, TokenType::Super);
        assert_token!(scanner, 33, TokenType::This);
        assert_token!(scanner, 34, TokenType::True);
        assert_token!(scanner, 35, TokenType::Var);
        assert_token!(scanner, 36, TokenType::While);
        assert_token!(scanner, 37, TokenType::Number(1.0));
        assert_token!(scanner, 38, TokenType::Slash);
        assert_token!(scanner, 39, TokenType::Number(3.0));
        assert_token!(scanner, 40, TokenType::Eof);
    }
}
