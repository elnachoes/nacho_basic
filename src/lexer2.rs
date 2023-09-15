#[derive(Hash, PartialEq, Eq)]
enum Token {
    // variable identity
    Identity(String),

    // literal declarations
    StringLiteral(String),
    IntLiteral(i32),
    FloatLiteral(f32),
    
    // operators
    Operator(Operator),

    // Type literal tokens for type annotations
    Type(Type),

    // Keywords
    KeyWord(Keyword),

    // KeyLimiters
    KeyLimiter(KeyLimiter),

    // Unexpected Token
    Error,
}

trait TokenLiteralMap<T> {
    fn get_token_literal_map(string : &str) -> T;
}

#[derive(Hash, PartialEq, Eq)]
enum Keyword {
    If,
    Elif,
    Else,
    While,
    For,
    Function,
}
impl TokenLiteralMap<Keyword> for Keyword {
    fn get_token_literal_map(string : &str) -> Keyword {
        match string {
            "if" => Keyword::If,
            "elif" => Keyword::Elif,
            "else" => Keyword::Else,
            "while" => Keyword::While,
            "for" => Keyword::For,
            "fn" => Keyword::Function,
            _ => panic!("lexing error : unexpected keyword")
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum KeyLimiter {
    Space,
    Quote,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
}
impl TokenLiteralMap<KeyLimiter> for KeyLimiter {
    fn get_token_literal_map(string : &str) -> KeyLimiter {
        match string {
            " " => KeyLimiter::Space,
            "\"" => KeyLimiter::Quote,
            "(" => KeyLimiter::OpenParen,
            ")" => KeyLimiter::CloseParen,
            "{" => KeyLimiter::OpenBracket,
            "}" => KeyLimiter::CloseBracket,
            _ => panic!("lexing error : unexpected key limiter")
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum Type {
    Int,
    Float,
    String,
}
impl TokenLiteralMap<Type> for Type {
    fn get_token_literal_map(string : &str) -> Type {
        match string {
            "int" => Type::Int,
            "float" => Type::Float,
            "string" => Type::String,
            _ => panic!("lexing error : unexpected type")
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide, 
    Mod,
    Assignment
}
impl TokenLiteralMap<Operator> for Operator {
    fn get_token_literal_map(string : &str) -> Operator {
        match string {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            "%" => Operator::Mod,
            "=" => Operator::Assignment,
            _ => panic!("lexing error : unexpected operator")
        }
    }
}

fn lex_file(file : &String) {
    
}