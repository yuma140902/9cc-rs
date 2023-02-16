//! コードジェネレータ
use crate::ast::{BinOp, Node};

/// 抽象構文木からアセンブリコードを作る
pub fn gen(node: &Node) {
    match node {
        Node::BinOp(op, lhs, rhs) => {
            gen(lhs);
            gen(rhs);
            println!("\tpop rdi");
            println!("\tpop rax");
            match *op {
                BinOp::Add => println!("\tadd rax, rdi"),
                BinOp::Sub => println!("\tsub rax, rdi"),
                BinOp::Mul => println!("\timul rax, rdi"),
                BinOp::Div => {
                    println!("\tcqo");
                    println!("\tidiv rdi");
                }
            }
            println!("\tpush rax");
        }
        Node::Unary(op, operand) => {
            gen(operand);
            println!("\tpop rax");
            match *op {
                crate::ast::UnaryOp::Positive => { /* nothing */ }
                crate::ast::UnaryOp::Negative => println!("\tneg rax"),
            }
            println!("\tpush rax");
        }
        Node::Num(num) => println!("\tpush {num}"),
    }
}
