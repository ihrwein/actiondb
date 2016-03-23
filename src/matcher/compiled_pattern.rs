use parsers::Parser;

pub type CompiledPattern = Vec<TokenType>;

#[derive(Debug)]
pub enum TokenType {
    Parser(Box<Parser>),
    Literal(String),
}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        match *self {
            TokenType::Parser(ref parser) => {
                TokenType::Parser(parser.boxed_clone())
            }
            TokenType::Literal(ref literal) => {
                TokenType::Literal(literal.clone())
            }
        }
    }
}

pub struct CompiledPatternBuilder {
    pattern: CompiledPattern,
}

impl CompiledPatternBuilder {
    pub fn new() -> CompiledPatternBuilder {
        CompiledPatternBuilder::default()
    }

    pub fn literal<S>(&mut self, literal: S) -> &mut CompiledPatternBuilder
        where S: Into<String>
    {
        self.pattern.push(TokenType::Literal(literal.into()));
        self
    }

    pub fn parser(&mut self, parser: Box<Parser>) -> &mut CompiledPatternBuilder {
        self.pattern.push(TokenType::Parser(parser));
        self
    }

    pub fn build(&self) -> CompiledPattern {
        self.pattern.clone()
    }
}

impl Default for CompiledPatternBuilder {
    fn default() -> Self {
        CompiledPatternBuilder { pattern: Vec::new() }
    }
}
