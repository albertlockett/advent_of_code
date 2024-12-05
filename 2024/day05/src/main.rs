use std::sync::Arc;

use arrow::{
    array::downcast_array,
    compute::kernels::bitwise::{bitwise_and, bitwise_shift_left},
    util::pretty::print_batches,
};
use arrow_array::{RecordBatch, UInt16Array, UInt8Array};
use arrow_schema::{DataType, Field, Schema};
use datafusion::prelude::SessionContext;

#[tokio::main]
async fn main() {
    let input_p1 = include_bytes!("../../inputs/day05/real_p1.txt");
    let input_p2 = include_bytes!("../../inputs/day05/real_p2.txt");

    let mut masks = vec![0u8; 100 * 13];

    let mut i = 0;
    while i < input_p1.len() {
        let l = (input_p1[i] - 48) * 10 + (input_p1[i + 1] - 48);
        let r = (input_p1[i + 3] - 48) * 10 + (input_p1[i + 4] - 48);

        let mask_offset = r / 8;
        let mask_bit = r % 8;

        masks[l as usize * 13 + mask_offset as usize] |= 1 << mask_bit;
        i += 6;
    }

    let mut i = 0;
    let mut update_rank = 0;

    let mut rank_builder = UInt16Array::builder(0);
    let mut lval_builder = UInt8Array::builder(0);
    let mut mask_bit_builder = UInt8Array::builder(0);
    let mut mask_builder = UInt8Array::builder(0);

    let mut mid_vals: Vec<u8> = vec![];
    let mut curr_update: Vec<u8> = vec![];

    loop {
        let l = (input_p2[i] - 48) * 10 + (input_p2[i + 1] - 48);
        i += 2;
        curr_update.push(l);

        if i >= input_p2.len() {
            break;
        }
        let c = input_p2[i];
        i += 1;
        match c {
            b',' => {
                let r = (input_p2[i] - 48) * 10 + (input_p2[i + 1] - 48);
                let mask_offset = r / 8;
                let mask_bit = r % 8;
                let mask = masks[l as usize * 13 + mask_offset as usize];
                rank_builder.append_value(update_rank);
                lval_builder.append_value(l);
                mask_bit_builder.append_value(mask_bit);
                mask_builder.append_value(mask);
            }
            b'\n' => {
                update_rank += 1;

                let mid_val = curr_update.get(curr_update.len() / 2);
                mid_vals.push(*mid_val.unwrap());
                curr_update.clear();
            }
            c => {
                println!("Invalid input {}", c);
                break;
            }
        }
    }

    let ranks = rank_builder.finish();
    let lvals = lval_builder.finish();
    let mask_bits = mask_bit_builder.finish();
    let masks = mask_builder.finish();

    let rule_check = bitwise_and(
        &masks,
        &bitwise_shift_left(&UInt8Array::from(vec![1u8; ranks.len()]), &mask_bits).unwrap(),
    )
    .unwrap();

    let ctx = SessionContext::new();

    let schema = Schema::new(vec![
        Field::new("rank", DataType::UInt16, false),
        Field::new("lvalue", DataType::UInt8, false),
        Field::new("ok", DataType::UInt8, false),
    ]);

    ctx.register_batch(
        "update_checks",
        RecordBatch::try_new(
            Arc::new(schema),
            vec![Arc::new(ranks), Arc::new(lvals), Arc::new(rule_check)],
        )
        .unwrap(),
    )
    .unwrap();

    let result = ctx
        .sql("select rank from update_checks group by rank having min(ok) > 0")
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();

    let p1_result_total: u32 = result
        .iter()
        .map(|batch| {
            downcast_array::<UInt16Array>(batch.column_by_name("rank").unwrap())
                .iter()
                .map(|rank| mid_vals[rank.unwrap() as usize] as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("part 1 = {}", p1_result_total);
}
