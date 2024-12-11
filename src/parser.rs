use crate::ast::{AssignmentExpr, BinaryExpr, Expr, Identifier, NodeType, NullLiteral, NumericLiteral, Program, Stmt, VarDeclaration};
use crate::lexer::{tokenize, Token, TokenType};
use crate::values::Value;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(source_code: String) -> Self {
        let tokens = tokenize(source_code);
        Parser { tokens, current: 0 }
    }

    fn not_eof(&self) -> bool {
        self.current < self.tokens.len() && self.tokens[self.current].ttype != TokenType::EOF
    }

    fn at(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn eat(&mut self) -> Token {
        self.current += 1;
        return self.tokens[self.current - 1].clone()
    }

    fn expect(&mut self, ttype: TokenType, err: String) -> Token {
        self.current += 1;
        let prev = self.tokens[self.current - 1].clone();
        if prev.tvalue == "" || prev.ttype != ttype {
            panic!("Parser Error:\n{}\nExpected: {:?}\nFound: {:?}", err, ttype, prev);
        }
        return prev
    }

    pub fn produce_ast(&mut self) -> Program {
        let mut program = Program {
            kind: NodeType::Program,  // Correctly setting the NodeType
            body: Vec::new(),
        };

        while self.not_eof() {
            program.body.push(self.parse_stmt());
        }
        program
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.at().ttype {
            TokenType::Let => {
                return self.parse_var_declaration()
            },
            TokenType::Const => {
                return self.parse_var_declaration()
            },
            _ => {
                return Stmt::Expr(self.parse_expr())                
            }
        }
        
    }
    fn parse_var_declaration(&mut self) -> Stmt {
        let is_constant = self.eat().ttype == TokenType::Const;
        let identifier = self.expect(
            TokenType::Identifier,
            String::from("Expected identifier name following let/const keywords.")
        ).tvalue;
        if self.at().ttype == TokenType::Semicolon {
            self.eat();//expect semicolon
            if is_constant {
                panic!("Must assign value to constant expression. No value provided")
            }
            return Stmt::VarDeclaration(VarDeclaration {
                kind: NodeType::VarDeclaration,
                constant: false,
                identifier,
                value: None,
            })
        }
        let is_constant_node = is_constant;
        self.expect(
            TokenType::Equals,
            String::from("Expected 'equals' token following identifier in var declaration")
        );
        let declaration = VarDeclaration {
            kind: NodeType::VarDeclaration,
            constant: is_constant_node,
            identifier,
            value: Some(self.parse_expr()),
        };
        self.expect(
            TokenType::Semicolon,
            String::from("Variable Declaration must end with semicolon.")
        );
        return Stmt::VarDeclaration(declaration)
    }

    fn parse_expr(&mut self) -> Expr {
        return self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> Expr {
        let left = self.parse_additive_expr();
        if self.at().ttype ==  TokenType::Equals {
            self.eat();
            let value = self.parse_assignment_expr();
            return Expr::AssignmentExpr(Box::new(AssignmentExpr {
                kind: NodeType::AssignmentExpr,
                assigne: left,
                value,
            }))
        }
        return left
    }

    fn parse_additive_expr(&mut self) -> Expr {
        let mut left = self.parse_multiplicative_expr();
        while self.at().tvalue == "+" || self.at().tvalue == "-" {
            let operator = self.eat().tvalue;
            let right = self.parse_multiplicative_expr();
            left = Expr::BinaryExpr(Box::new(BinaryExpr {
                kind: NodeType::BinaryExpr,
                left,
                right,
                operator,
            }));
        }
        return left
    }

    fn parse_multiplicative_expr(&mut self) -> Expr {
        let mut left = self.parse_primary_expr();
        while self.at().tvalue == "/" || self.at().tvalue == "*" || self.at().tvalue == "%" { // might need to do modulo
            let operator = self.eat().tvalue;
            let right = self.parse_primary_expr();
            left = Expr::BinaryExpr(Box::new(BinaryExpr {
                kind: NodeType::BinaryExpr,
                left,
                right,
                operator,
            }));
        }
        return left
    }
        /* `Expr` value */
/*order of precidence
    assignmentExpr
    memberExpr
    functionCall
    logicalExpr
    comparisonExpr
    additiveExpr
    multiplicativeExpr
    unaryExpr
    primaryExpr */

    fn parse_primary_expr(&mut self) -> Expr {
        match self.at().ttype {
            TokenType::Number => {
                let value = self.at().tvalue.clone();
                self.eat();
                Expr::NumericLiteral(NumericLiteral {
                    kind: NodeType::NumericLiteral,
                    value: value.parse::<i64>().unwrap(),
                })
            }
            TokenType::Null => {
                self.eat();
                Expr::NullLiteral(NullLiteral {
                    kind: NodeType::NullLiteral,
                    value: Value::Null,
                })
            }
            TokenType::Identifier => {
                let symbol = self.at().tvalue.clone();
                self.eat();
                Expr::Identifier(Identifier {
                    kind: NodeType::Identifier,
                    symbol,
                })
            }
            TokenType::OpenParen => {
                self.eat();
                let value = self.parse_expr();
                self.expect(
                    TokenType::CloseParen,
                    String::from("Unexpected token found inside parenthesized expression.")
                );
                return value
            }
            _ => {
                panic!("Unexpected token type: {:?}", self.at());
            }
        }
    }
}