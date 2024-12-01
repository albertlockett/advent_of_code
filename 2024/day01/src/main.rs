use arrow::array::downcast_array;
use arrow::compute::{sort, SortOptions};
use arrow_array::{Int32Array, Int64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use datafusion::execution::context::SessionContext;
use plex::lexer;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use aoc::core::{error::Result, parser::Lexer};

#[derive(Debug, Clone)]
pub enum Token {
    Whitespace,
    Location(i32),
}

lexer! {
    pub fn next_token(text: 'a) -> Token;

    r#"[ \n]+"# => Token::Whitespace,
    r#"[0-9]+"# => {
        if let Ok(i) = text.parse() {
            Token::Location(i)
        } else {
            panic!("integer {} is out of range", text)
        }
    }
}

async fn do_it(input: &str) -> Result<i64> {
    let file = File::open(format!("inputs/day01/{}.txt", input))
        .await
        .unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut loc_builder_l = Int32Array::builder(2);
    let mut loc_builder_r = Int32Array::builder(2);

    while let Some(line) = lines.next_line().await? {
        let f = |e| next_token(e);
        let mut lexer = Lexer::<Token>::new(&line, Box::new(f));
        match (lexer.next(), lexer.next(), lexer.next()) {
            (Some(Token::Location(left)), _, Some(Token::Location(right))) => {
                loc_builder_l.append_value(left);
                loc_builder_r.append_value(right);
            }
            _ => {
                panic!("invalid line {}", line)
            }
        }
    }

    let sort_options = SortOptions {
        descending: false,
        ..Default::default()
    };
    let locs_l = sort(&loc_builder_l.finish(), Some(sort_options))?;
    let locs_r = sort(&loc_builder_r.finish(), Some(sort_options))?;

    let schema = Arc::new(Schema::new(vec![
        Field::new("loc_l", DataType::Int32, false),
        Field::new("loc_r", DataType::Int32, false),
    ]));

    let record_batch =
        RecordBatch::try_new(schema.clone(), vec![Arc::new(locs_l), Arc::new(locs_r)])?;

    let ctx = SessionContext::new();
    ctx.register_batch("input", record_batch)?;

    let result = ctx
        .sql("select sum(abs(loc_l - loc_r)) from input")
        .await
        .unwrap()
        .collect()
        .await?;
    let locs_diff = downcast_array::<Int64Array>(result[0].column(0)).value(0);

    Ok(locs_diff)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", do_it("test").await?);
    println!("{}", do_it("real").await?);
    Ok(())
}
