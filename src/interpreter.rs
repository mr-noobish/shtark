use crate::values::{NullVal, NumberVal, RuntimeVal, ValueType, Value};
use crate::ast::{AssignmentExpr, BinaryExpr, Expr, Identifier, Program, Stmt, VarDeclaration};
use crate::environment::Environment;

pub fn eval_binary_expr(binop: BinaryExpr, env: &mut Environment) -> RuntimeVal {
    let lhs = eval_expr(binop.left, env);
    let rhs = eval_expr(binop.right, env);
    if lhs.get_value_type() == ValueType::Number && rhs.get_value_type() == ValueType::Number {
        return eval_numeric_binary_expr(lhs.to_number_val(), rhs.to_number_val(), binop.operator).to_runtime_val()
    } else {
        return RuntimeVal::NullVal(NullVal {
            value_type: ValueType::Null,
            value: Value::Null
        })
    }
}

fn eval_numeric_binary_expr(lhs: NumberVal, rhs: NumberVal, operator: String) -> NumberVal {
    let mut result = 0;
    if operator == "+" {
        result = lhs.value + rhs.value
    } else if operator == "-" {
        result = lhs.value - rhs.value
    } else if operator == "*" {
        result = lhs.value * rhs.value
    } else if operator == "/" { //TODO check for divide by zero
        result = lhs.value / rhs.value
    } else if operator == "%" {
        result = lhs.value % rhs.value
    }
    return NumberVal {
        value_type: ValueType::Number,
        value: result
    }
}
fn eval_identifier(ident: Identifier, env: &mut Environment) -> RuntimeVal {
    let val = env.lookup_var(ident.symbol);
    return val
}

fn eval_program(program: Program, env: &mut Environment) -> RuntimeVal {
    let mut last_evaluated: RuntimeVal = RuntimeVal::NullVal(NullVal {
        value_type: ValueType::Null,
        value: Value::Null
    });
    for statement in program.body {
        last_evaluated = evaluate(statement, env);
    }
    return  last_evaluated
}

fn eval_expr(ast_node: Expr, env: &mut Environment) -> RuntimeVal {
    if let Expr::NumericLiteral(value) = ast_node {
        return RuntimeVal::NumberVal(NumberVal {
            value_type: ValueType::Number,
            value: value.value,
        }) 
    } else if let Expr::NullLiteral(_null_literal) = ast_node {
        return RuntimeVal::NullVal(NullVal {
            value_type: ValueType::Null,
            value: Value::Null
        })
    } else if let Expr::BinaryExpr(binary_expr) = ast_node {
        return eval_binary_expr(*binary_expr, env)
    } else if let Expr::Identifier(identifier) = ast_node {
        return eval_identifier(identifier, env)
    } else if let Expr::AssignmentExpr(assignment_expr) = ast_node {
        return eval_assignment(*assignment_expr, env)
    } else {
        panic!("this ast node has not been implemented yet:\n{:#?}", ast_node)
    }
}

fn eval_var_declaration(declaration: VarDeclaration, env: &mut Environment) -> RuntimeVal {
    match declaration.value {
        Some(_) => {
            return env.declare_var(declaration.identifier, declaration.value.expect("huh").expr_to_runtime_val(&mut env.clone()), false)
        },//whyyyyyyyyyyyyy is it not added to the hashmap
        None => return env.declare_var(declaration.identifier, RuntimeVal::NullVal(NullVal {
            value_type: ValueType::Null,
            value: Value::Null,
        }), false),
    }
}
fn eval_const_declaration(declaration: VarDeclaration, env: &mut Environment) -> RuntimeVal {
    match declaration.value {
        Some(_) => {
            return env.declare_var(declaration.identifier, declaration.value.expect("huh").expr_to_runtime_val(&mut env.clone()), true)
        },
        None => return env.declare_var(declaration.identifier, RuntimeVal::NullVal(NullVal {
            value_type: ValueType::Null,
            value: Value::Null,
        }), true),
    }
}


pub fn evaluate(ast_node: Stmt, env: &mut Environment) -> RuntimeVal {
    match ast_node {
        Stmt::Program(program) => return eval_program(program, env),
        Stmt::VarDeclaration(var_declaration) => {
            match var_declaration.constant {
                true => return eval_const_declaration(var_declaration, env),
                false => return eval_var_declaration(var_declaration, env),
            }
        },
        Stmt::Expr(_) => return eval_expr(ast_node.get_stmt_expr(), env),
    };
}

fn eval_assignment(node: AssignmentExpr, env: &mut Environment) -> RuntimeVal {
    match node.assigne {
        Expr::Identifier(identifier) => {
            let varname = identifier.symbol.clone();
            let value = evaluate(Stmt::Expr(node.value), env);
            return env.assign_var(varname, value)
        },
        _ => {
            panic!("Invalid LHS inside assignment expression:\n{:#?}", node.assigne)
        },
    }
}