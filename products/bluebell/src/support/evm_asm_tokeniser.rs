enum Token {
    Mstore,
    Callvalue,
    Dup1,
    Iszero,
    Tag,
    Jumpi,
    Revert,
    Pop,
    DataSize,
    DataOffset,
    Codecopy,
    Return,
    Stop,
    Sub,
    Assembly,
    Auxdata,
    Number(u64),
    HexNumber(Vec<u8>),
    Identifier(String),
}

pub fn tokenise(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for word in input.split_whitespace() {
        let token = match word {
            "mstore" => Token::Mstore,
            "callvalue" => Token::Callvalue,
            "dup1" => Token::Dup1,
            "iszero" => Token::Iszero,
            "tag" => Token::Tag,
            "jumpi" => Token::Jumpi,
            "revert" => Token::Revert,
            "pop" => Token::Pop,
            "dataSize" => Token::DataSize,
            "dataOffset" => Token::DataOffset,
            "codecopy" => Token::Codecopy,
            "return" => Token::Return,
            "stop" => Token::Stop,
            "sub" => Token::Sub,
            "assembly" => Token::Assembly,
            "auxdata" => Token::Auxdata,
            _ => {
                if let Ok(number) = word.parse::<u64>() {
                    Token::Number(number)
                } else if word.starts_with("0x") {
                    Token::HexNumber(hex::decode(&word[2..]).unwrap())
                } else {
                    Token::Identifier(word.to_string())
                }
            }
        };
        tokens.push(token);
    }

    tokens
}
