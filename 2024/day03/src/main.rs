use std::sync::Arc;

use arrow::datatypes::{DataType, Field, Schema};
use arrow_array::{Int32Array, RecordBatch};
use datafusion::prelude::SessionContext;
use plex::lexer;
use tokio::{fs::File, io::{AsyncBufReadExt, BufReader}};

use aoc::core::parser::{numberic::{self}, Lexer};

#[derive(Debug)]
enum Token {
    Crap,
    Mul(i32, i32),
    Do,
    Dont,
}

lexer! {
    pub fn next_token(text: 'a) -> Token;

    r#"do\(\)"# => Token::Do,
    r#"don't\(\)"# => Token::Dont,
    r#"mul\([0-9]+,[0-9]+\)"# => {
        let mut lexer = Lexer::<numberic::Token>::new(text, Box::new(numberic::next_token));
        match (
            lexer.next().unwrap(), // m
            lexer.next().unwrap(), // u
            lexer.next().unwrap(), // l
            lexer.next().unwrap(), // (
            lexer.next().unwrap(), // x1
            lexer.next().unwrap(), // ,
            lexer.next().unwrap(), // x2
            lexer.next().unwrap(), // )
        ) {
            (
                numberic::Token::Other,
                numberic::Token::Other,
                numberic::Token::Other,
                numberic::Token::Other,
                numberic::Token::Number(i1),
                numberic::Token::Other,
                numberic::Token::Number(i2),
                numberic::Token::Other,
            ) => {
                Token::Mul(i1, i2)
            }
            _ => {
                panic!("bad text {}", text)
            }
        }
        
    },
    "." => Token::Crap
}

#[tokio::main]
async fn main() {
    let mut lines = BufReader::new(File::open("./inputs/day03/real.txt").await.unwrap()).lines();
    let mut left_builder = Int32Array::builder(0);
    let mut right_builder = Int32Array::builder(0);
    let mut mult_enabled = true;
    while let Some(line) = lines.next_line().await.unwrap() {
        for token in Lexer::<Token>::new(&line, Box::new(next_token)) {
            match token {
                Token::Mul(i1,i2) => {
                    if mult_enabled {
                        left_builder.append_value(i1);
                        right_builder.append_value(i2);
                    }
                },
                Token::Do => mult_enabled = true,
                Token::Dont => mult_enabled = false,
                _ => {}
            }
        }
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("left", DataType::Int32, false),
        Field::new("right", DataType::Int32, false)
    ]));

    let record_batch = RecordBatch::try_new(schema.clone(), vec![
        Arc::new(left_builder.finish()),
        Arc::new(right_builder.finish())
    ]).unwrap();

    let ctx = SessionContext::new();
    ctx.register_batch("input", record_batch).unwrap();

    let result = ctx.sql("select sum(left * right) from input")
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();

    println!("{:?}", result);
}
