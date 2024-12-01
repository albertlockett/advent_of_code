use std::sync::Arc;
use arrow::array::downcast_array;
use arrow::compute::{sort, SortOptions};
use arrow_array::{Int32Array, Int64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use datafusion::execution::context::SessionContext;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};



use aoc::core::error::Result;
use input::Token;

mod input;

#[tokio::main]
async fn main() -> Result<()> {
    let file = File::open("inputs/day01/real.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut loc_builder_l = Int32Array::builder(2);
    let mut loc_builder_r = Int32Array::builder(2);

    while let Some(line) = lines.next_line().await? {
        let mut lexer = input::Lexer::new(&line);
        match (lexer.next(), lexer.next(), lexer.next()) {
            (Some(Token::Location(left)), _, Some(Token::Location(right))) => {
                loc_builder_l.append_value(left);
                loc_builder_r.append_value(right);
            },
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

    let record_batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(locs_l), Arc::new(locs_r)])?;

    let ctx = SessionContext::new();
    ctx.register_batch("input", record_batch)?;

    let result = ctx.sql("select sum(abs(loc_l - loc_r)) from input").await.unwrap().collect().await?;
    let locs_diff = downcast_array::<Int64Array>(result[0].column(0)).value(0);
    println!("part1 answer = {:?}", locs_diff);

    Ok(())
}
