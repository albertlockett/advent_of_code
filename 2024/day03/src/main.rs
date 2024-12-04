use std::sync::Arc;

use arrow::datatypes::{DataType, Field, Schema};
use arrow_array::{Int32Array, RecordBatch};
use datafusion::prelude::SessionContext;
use plex::{lexer, parser};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use aoc::core::parser::{Lexer, Span};

#[derive(Debug, PartialEq)]
enum Token {
    Crap,
    Mul,
    Number(i32),
    Comma,
    OpenParan,
    CloseParen,
    Do,
    Dont,
}

lexer! {
    pub fn next_token(text: 'a) -> Token;

    r#"do"# => Token::Do,
    r#"don't"# => Token::Dont,
    r#"[0-9]+"# => {
        Token::Number(text.parse().unwrap())
    }
    r#"mul"# => Token::Mul,
    r#"\("# => Token::CloseParen,
    r#"\)"# => Token::CloseParen,
    r#","# => Token::Comma,
    "." => Token::Crap
}

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Expr>,
}

#[derive(Debug)]
pub struct Expr {
    pub span: Span,
    pub node: Expr_,
}

#[derive(Debug)]
pub enum Expr_ {
    Mul(i32, i32),
    Do,
    Dont,
}

mod parser {
    use super::Token::*;
    use super::*;

    parser! {
        fn parse_(Token, Span);

        // combine two spans
        (a, b) {
            Span {
                lo: a.lo,
                hi: b.hi,
            }
        }

        program: Program {
            statements[s] => Program { stmts: s }
        }

        statements: Vec<Expr> {
            => vec![],
            statements[mut st]  cmd[e] => {
                if let Some(e) = e {
                    st.push(Expr{ node: e, span: span!() });
                }
                st
            }
        }

        cmd: Option<Expr_> {
         crap => None,
         multexp[e] => Some(e),
            Do OpenParan CloseParen => Some(Expr_::Do),
            Dont OpenParan CloseParen => Some(Expr_::Dont),
        }

        multexp: Expr_ {
            Mul OpenParan Number(i1) Comma Number(i2) CloseParen => Expr_::Mul(i1, i2),
        }

        crap: () {
            Crap => (),
        }

    }

    pub fn parse<I: Iterator<Item = (Token, Span)>>(
        i: I,
    ) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
        parse_(i)
    }
}

#[tokio::main]
async fn main() {
    let mut lines = BufReader::new(File::open("./inputs/day03/real.txt").await.unwrap()).lines();
    // let input = include_str!("../../inputs/day03/real.txt");
    // let mut lines = input.lines();
    let mut left_builder = Int32Array::builder(0);
    let mut right_builder = Int32Array::builder(0);
    let mut mult_enabled = true;

    while let Some(line) = lines.next_line().await.unwrap() {
        // while let Some(line) = lines.next() {
        let lexer = Lexer::<Token>::new(&line, Box::new(next_token))
            .into_iter()
            .filter(|(t, _)| *t != Token::Crap);
        let program = parser::parse(lexer).unwrap();
        for stmt in program.stmts {
            println!("{:?}", stmt);
        }
        // for token in Lexer::<Token>::new(&line, Box::new(next_token)) {
        //         match token {
        //             Token::Mul(i1, i2) => {
        //                 if mult_enabled {
        //                     left_builder.append_value(i1);
        //                     right_builder.append_value(i2);
        //                 }
        //             }
        //             Token::Do => mult_enabled = true,
        //             Token::Dont => mult_enabled = false,
        //             _ => {}
        //         }
        // }
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("left", DataType::Int32, false),
        Field::new("right", DataType::Int32, false),
    ]));

    let record_batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(left_builder.finish()),
            Arc::new(right_builder.finish()),
        ],
    )
    .unwrap();

    let ctx = SessionContext::new();
    ctx.register_batch("input", record_batch).unwrap();

    let result = ctx
        .sql("select sum(left * right) from input")
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();

    println!("{:?}", result);
}
