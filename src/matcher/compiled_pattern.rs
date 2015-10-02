use parsers::Parser;

pub type CompiledPattern = Vec<TokenType>;

#[derive(Debug)]
pub enum TokenType {
    Parser(Box<Parser>),
    Literal(String)
}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        match self {
            &TokenType::Parser(ref parser) => {
                TokenType::Parser(parser.boxed_clone())
            },
            &TokenType::Literal(ref literal) => {
                TokenType::Literal(literal.clone())
            }
        }
    }
}
