use crate::lexer::Token;

pub enum ASTNode {
    FunctionDeclaration {
        name: String,
        args: Vec<String>,
        body: Vec<ASTNode>
    },
    VariableDeclaration {
        name: String,
        value: Box<ASTNode>
    },
    If {
        condition: Box<ASTNode>,
        then: Vec<  ASTNode>,
        r#else: Option<Box<ASTNode>>
    },
    For {
        condition: Box<ASTNode>,
        body: Box<ASTNode>
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut tokens_iter = tokens.into_iter().peekable();
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Let => {
                if let Some(Token::Identifier(name)) = tokens_iter.next() {
                    if tokens_iter.next() != Some(Token::Equals) {
                        panic!("Expected = after variable name");
                    }
                }
            },
            _ => todo!(),
        }
    }
    ast
}