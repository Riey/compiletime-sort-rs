extern crate proc_macro;

use syn::{parse_macro_input, BinOp, Expr, ExprBinary, ExprLit, ExprUnary, ItemConst, Lit, UnOp};

use proc_macro::TokenStream;

fn bin_op(op: BinOp, left: i64, right: i64) -> i64 {
    match op {
        BinOp::BitOr(_) => left | right,
        BinOp::BitXor(_) => left ^ right,
        BinOp::BitAnd(_) => left & right,
        BinOp::Add(_) => left + right,
        BinOp::Sub(_) => left - right,
        BinOp::Mul(_) => left * right,
        BinOp::Div(_) => left / right,
        BinOp::Rem(_) => left % right,
        BinOp::Shl(_) => left << right,
        BinOp::Shr(_) => left >> right,
        _ => panic!("Invalie BinOp"),
    }
}

fn un_op(op: UnOp, expr: i64) -> i64 {
    match op {
        UnOp::Not(_) => !expr,
        UnOp::Neg(_) => -expr,
        _ => panic!("Invalie UnOp"),
    }
}

fn eval_expr(expr: Expr) -> i64 {
    match expr {
        Expr::Lit(ExprLit { lit: Lit::Int(num), .. }) => num.base10_parse().unwrap(),
        Expr::Binary(ExprBinary { op, left, right, .. }) => bin_op(op, eval_expr(*left), eval_expr(*right)),
        Expr::Unary(ExprUnary { op, expr, .. }) => un_op(op, eval_expr(*expr)),
        _ => panic!("Invalid num expr"),
    }
}

#[proc_macro_attribute]
pub fn sort(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let arr = parse_macro_input!(item as ItemConst);
    let arr = match *arr.expr {
        Expr::Array(arr) => arr,
        _ => panic!("Const expr is must array"),
    };

    let mut ret = Vec::with_capacity(arr.elems.len());

    for elem in arr.elems {
        ret.push(eval_expr(elem));
    }

    ret.sort_unstable();

    panic!("{:?}", ret);
}

