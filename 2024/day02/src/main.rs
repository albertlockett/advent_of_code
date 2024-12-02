use std::io::BufRead;
use std::sync::Arc;
use std::{
    fs::File,
    io::{BufReader, Lines},
};

use arrow::compute::{max, min};
use arrow_array::builder::{ArrayBuilder, GenericListBuilder, ListBuilder};
use arrow_array::cast::{as_list_array, downcast_array};
use arrow_array::{
    ArrayRef, BooleanArray, GenericListArray, Int32Array, ListArray, RecordBatch,
    RecordBatchIterator,
};
use arrow_schema::{DataType, Field, Schema};
use datafusion::execution::context::SessionContext;
use datafusion::logical_expr::ColumnarValue;
use datafusion::prelude::*;

// use tokio::{
//     fs::File, io::{AsyncBufReadExt, BufReader, Lines}
// };

use aoc::core::parser::{
    numberic::{next_token, Token},
    Lexer,
};

struct InputIter {
    schema: Arc<Schema>,
    bath_size: usize,
    lines: Lines<BufReader<File>>,
}

impl Iterator for InputIter {
    type Item = RecordBatch;

    fn next(&mut self) -> Option<RecordBatch> {
        let mut list_array_builder = ListBuilder::new(Int32Array::builder(2));

        for _ in 0..self.bath_size {
            let line = match self.lines.next() {
                Some(line) => line.unwrap(),
                None => break,
            };
            let lexer = Lexer::<Token>::new(&line, Box::new(next_token));

            for token in lexer {
                match token {
                    Token::Number(i) => list_array_builder.values().append_value(i),
                    _ => {}
                }
            }
            list_array_builder.append(true)
        }

        if list_array_builder.len() == 0 {
            None
        } else {
            Some(
                RecordBatch::try_new(
                    self.schema.clone(),
                    vec![Arc::new(list_array_builder.finish())],
                )
                .unwrap(),
            )
        }
    }
}

fn schema() -> Schema {
    Schema::new(vec![Field::new(
        "reports",
        DataType::List(Arc::new(Field::new("item", DataType::Int32, true))),
        false,
    )])
}

#[tokio::main]
async fn main() {
    let mut file = File::open("inputs/day02/real.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let schema = Arc::new(schema());
    let input_iter = InputIter {
        schema,
        lines,
        bath_size: 10,
    };

    let checker = Arc::new(|args: &[ColumnarValue]| {
        let mut result_builder = BooleanArray::builder(0);
        let args = ColumnarValue::values_to_arrays(args)?;
        let records = as_list_array(&args[0]);

        for record in records.iter() {
            let vals = downcast_array::<Int32Array>(&record.unwrap());
            let mut diffs = Int32Array::builder(0);
            for i in 1..vals.len() {
                diffs.append_value(vals.value(i) - vals.value(i - 1));
            }

            let diffs = diffs.finish();
            let max = max(&diffs).unwrap();
            let min = min(&diffs).unwrap();
            let safe = (min > 0 && max <= 3) || (max < 0 && min >= -3);
            result_builder.append_value(safe);
        }

        let result = result_builder.finish();

        Ok(ColumnarValue::from(Arc::new(result) as ArrayRef))
    });

    let udf = create_udf(
        "report_check",
        vec![DataType::List(Arc::new(Field::new(
            "item",
            DataType::Int32,
            false,
        )))],
        DataType::Boolean,
        datafusion::logical_expr::Volatility::Immutable,
        checker,
    );

    let ctx = SessionContext::new();
    ctx.register_udf(udf.clone());
    let expr = udf.call(vec![col("reports")]);
    let df = ctx.read_batches(input_iter).unwrap();

    let result = df.filter(expr).unwrap().count().await.unwrap();
    print!("{:?}", result);
}
