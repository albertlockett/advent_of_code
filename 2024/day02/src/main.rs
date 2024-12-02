use std::io::BufRead;
use std::sync::Arc;
use std::{
    fs::File,
    io::{BufReader, Lines},
};

use arrow::compute::filter;
use arrow::compute::{kernels::numeric::sub, max, min};
use arrow_array::builder::{ArrayBuilder, ListBuilder};
use arrow_array::cast::{as_list_array, downcast_array};
use arrow_array::{ArrayRef, BooleanArray, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use datafusion::execution::context::SessionContext;
use datafusion::logical_expr::ColumnarValue;
use datafusion::prelude::*;

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
                if let Token::Number(i) = token {
                    list_array_builder.values().append_value(i)
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
    let schema = Arc::new(schema());
    let input_iter = InputIter {
        schema,
        lines: BufReader::new(File::open("inputs/day02/real.txt").unwrap()).lines(),
        bath_size: 10,
    };

    let checker = Arc::new(|args: &[ColumnarValue]| {
        let mut result_builder = BooleanArray::builder(0);
        let args = ColumnarValue::values_to_arrays(args)?;
        let records = as_list_array(&args[0]);

        for record in records.iter() {
            let record = record.unwrap();
            let vals = downcast_array::<Int32Array>(&record);
            let diffs = sub(
                &vals.slice(1, vals.len() - 1),
                &vals.slice(0, vals.len() - 1),
            )
            .unwrap();

            let diffs = downcast_array::<Int32Array>(&diffs);
            let max_val = max(&diffs).unwrap();
            let min_val = min(&diffs).unwrap();
            let mut safe = (min_val > 0 && max_val <= 3) || (max_val < 0 && min_val >= -3);

            // part 2
            let mut i = 0;
            while !safe && i < vals.len() {
                let mut mask_builder = BooleanArray::builder(0);
                for j in 0..vals.len() {
                    mask_builder.append_value(i != j);
                }
                let mask = mask_builder.finish();
                let vals = downcast_array::<Int32Array>(&filter(&vals, &mask).unwrap());
                let diffs = sub(
                    &vals.slice(1, vals.len() - 1),
                    &vals.slice(0, vals.len() - 1),
                )
                .unwrap();
                let diffs = downcast_array::<Int32Array>(&diffs);
                let max_val = max(&diffs).unwrap();
                let min_val = min(&diffs).unwrap();
                safe = (min_val > 0 && max_val <= 3) || (max_val < 0 && min_val >= -3);
                i += 1;
            }

            result_builder.append_value(safe);
        }

        Ok(ColumnarValue::from(
            Arc::new(result_builder.finish()) as ArrayRef
        ))
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
    let result = ctx
        .read_batches(input_iter)
        .unwrap()
        .filter(udf.call(vec![col("reports")]))
        .unwrap()
        .count()
        .await
        .unwrap();
    print!("{:?}", result);
}
