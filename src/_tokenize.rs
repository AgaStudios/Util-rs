pub struct Position {
    pub line: usize,
    pub column: usize,
}
impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            line: self.line,
            column: self.column,
        }
    }
}
impl Copy for Position {}

pub struct Token<TokenKind> {
    pub token_type: TokenKind,
    pub value: String,
    pub position: Position,
    pub meta: String,
}

pub type TokenOptionsCallback<TK> = fn(char, Position, String) -> (Token<TK>, usize);

pub enum TokenOptionCondition {
    Chars(&'static str),
    Fn(fn(char) -> bool),
}

pub type TokenOption<'a, TK> = (TokenOptionCondition, TokenOptionsCallback<TK>);

pub fn tokenize<TK>(
    input: String,
    options: Vec<TokenOption<TK>>,
) -> Result<Vec<Token<TK>>, Box<dyn std::error::Error>> {
    let lines = input.lines();
    let mut tokens = Vec::new();
    for (line_number, line) in lines.enumerate() {
        let mut column = 0;
        for c in line.chars() {
            let mut token: Option<Token<TK>> = None;
            for (condition, result) in &options {
                let is_valid = match condition {
                    TokenOptionCondition::Chars(chars) => chars.contains(c),
                    TokenOptionCondition::Fn(f) => f(c),
                };
                if !is_valid {
                    continue;
                }
                let position = Position {
                    line: line_number,
                    column,
                };
                let (t, consumed) = result(c, position, line.to_string());
                token = Some(t);
                column += consumed;
                break;
            }
            if let Some(token) = token {
                tokens.push(token);
            } else {
                return Err(format!("'{}'", c).into());
            }
            column += 1;
        }
    }
    Ok(tokens)
}
