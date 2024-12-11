use crate::{environment::Environment, interpreter::eval_binary_expr, values::{NullVal, NumberVal, RuntimeVal, Value, ValueType}};

#[derive(Debug, Clone)]
pub enum NodeType {
    //statements
    Program,
    VarDeclaration,

    //expressions
    AssignmentExpr,
    NumericLiteral,
    NullLiteral,
    Identifier,
    BinaryExpr,
    //ExprStmt,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Stmt>,
}
#[derive(Debug, Clone)]
pub struct VarDeclaration {
    pub kind: NodeType,
    pub constant: bool,
    pub identifier: String,
    pub value: Option<Expr>,
}
#[derive(Debug, Clone)]
pub enum Stmt {
    Program(Program),
    VarDeclaration(VarDeclaration),
    Expr(Expr),
}
impl Stmt {
    pub fn get_stmt_expr(&self) -> Expr {
        match self {
            Stmt::Expr(expr) => {
                match expr {
                    Expr::NumericLiteral(numeric_literal) => {
                        return Expr::NumericLiteral(NumericLiteral {
                            kind: NodeType::NumericLiteral,
                            value: numeric_literal.value,
                        })
                    },
                    Expr::NullLiteral(_) => {
                        return Expr::NullLiteral(NullLiteral {
                            kind: NodeType::NullLiteral,
                            value: Value::Null
                        })
                    },
                    Expr::Identifier(identifier) => {
                        return Expr::Identifier(Identifier {
                            kind: NodeType::Identifier,
                            symbol: identifier.symbol.clone()
                        })
                    },
                    Expr::BinaryExpr(binary_expr) => {
                        return Expr::BinaryExpr(Box::new(BinaryExpr {
                            kind: NodeType::BinaryExpr,
                            left: binary_expr.left.clone(),
                            right: binary_expr.right.clone(),
                            operator: binary_expr.operator.clone(),
                        }))
                    },
                    Expr::AssignmentExpr(assignment_expr) => {
                        return Expr::AssignmentExpr(Box::new(AssignmentExpr {
                            kind: NodeType::AssignmentExpr,
                            assigne: assignment_expr.assigne.clone(),
                            value: assignment_expr.value.clone(),
                        }))
                    },// get to this soon
                }
            },
            //Stmt::Program(program) => panic!("its a program"),                            //literally dont know why this wont work
            //Stmt::VarDeclaration(var_declaration) => (panic!("its a vardec")),      //remember to do this before moving on you bum
            _ => panic!("This statement is not an expression.")
        }
    }
}
#[derive(Debug, Clone)]
pub enum Expr {
    AssignmentExpr(Box<AssignmentExpr>),
    NumericLiteral(NumericLiteral),
    NullLiteral(NullLiteral),
    Identifier(Identifier),
    BinaryExpr(Box<BinaryExpr>),
}

impl Expr {
    pub fn to_binaryexpr(&self) -> BinaryExpr {
        match self {
            Expr::BinaryExpr(binary_expr) => return *binary_expr.clone(),
            _ => panic!("Expression is not a binary expression"),
        }
    }
    pub fn expr_to_runtime_val(&self, env: &mut Environment) -> RuntimeVal {
        match self {
            Expr::NumericLiteral(numeric_literal) => RuntimeVal::NumberVal(NumberVal {
                value_type: ValueType::Number,
                value: numeric_literal.value,
            }),
            Expr::NullLiteral(null_literal) => RuntimeVal::NullVal(NullVal {
                value_type: ValueType::Null,
                value: null_literal.value.clone(),
            }),
            Expr::Identifier(identifier) => panic!("its an identifier"),
            Expr::BinaryExpr(binary_expr) => RuntimeVal::NumberVal(NumberVal {
                value_type: ValueType::Number,
                value: eval_binary_expr(binary_expr.as_ref().clone(), env).to_number_val().value,
            }),
            Expr::AssignmentExpr(assignment_expr) => todo!(),//fix this too
            //_ => panic!("probably something else")
        }
    }
}
#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub kind: NodeType,
    pub assigne: Expr,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: Expr,
    pub right: Expr,
    pub operator: String,
}
#[derive(Debug, Clone)]
pub struct Identifier {
    pub kind: NodeType,
    pub symbol: String,
}
#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub kind: NodeType,
    pub value: i64,
}
#[derive(Debug, Clone)]
pub struct NullLiteral {
    pub kind: NodeType,
    pub value: Value,
}
