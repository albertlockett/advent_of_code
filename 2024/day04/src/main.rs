use std::sync::Arc;

use arrow_array::{
    builder::{ListBuilder, StringBuilder},
    RecordBatch,
};
use arrow_schema::{DataType, Field, Schema};
use datafusion::{arrow::util::pretty::print_batches, prelude::*};
use datafusion::{common::utils::transpose, prelude::SessionContext};

#[tokio::main]
async fn main() {
    let input = include_str!("../../inputs/day04/real.txt");
    let grid = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|e| *e != "")
                .into_iter()
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<_>>();

    let ctx = SessionContext::new();
    let schema = Arc::new(Schema::new(vec![Field::new(
        "s",
        DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))),
        false,
    )]));

    let mut list_builder = ListBuilder::new(StringBuilder::new());
    grid.iter()
        .for_each(|row| list_builder.append_value(row.iter().map(Some).collect::<Vec<_>>()));
    let batch =
        RecordBatch::try_new(schema.clone(), vec![Arc::new(list_builder.finish())]).unwrap();
    ctx.register_batch("grid", batch).unwrap();

    // let grid_t = transpose(grid);
    // let mut list_builder = ListBuilder::new(StringBuilder::new());
    // grid_t
    //     .iter()
    //     .for_each(|row| list_builder.append_value(row.iter().rev().map(Some).collect::<Vec<_>>()));
    // let batch =
    //     RecordBatch::try_new(schema.clone(), vec![Arc::new(list_builder.finish())]).unwrap();
    // ctx.register_batch("grid_t", batch).unwrap();

    // let tmp_view_def = ;
    let coords_view_def = "select distinct s, y, (row_number() OVER (PARTITION BY 'a') - 1) % (max(rows_tmp.y) + 1) as x";
    let view_defs = vec![
        "create view rows_tmp as select unnest(s) as s, row_number() OVER (PARTITION BY 'a') - 1 as y from grid",
        // format!("create view cols_tmp as {} from grid_t", tmp_view_def),
        // format!(
        "create view grid_coords as select distinct s, y, (row_number() OVER (PARTITION BY 'a') - 1) % 10 as x from rows_tmp order by y",
            // coords_view_def
        // ),
        // format!(
        //     "create view grid_t_coords as {} from cols_tmp order by y",
        //     coords_view_def
        // ),
        // "create view diag1 as select array_agg(s order by y asc) as s, x + y as sum from grid_coords group by x + y order by x + y".to_string(),
        // "create view diag2 as select array_agg(s order by y desc) as s, x + y as sum from grid_t_coords group by x + y order by x + y"
        //     .to_string(),
        // "create view text_fwd as \
        //     select array_join(s, '') as s, 'g1' as g from grid \
        //     union (select array_join(s, '') as s, 'gt' as g from grid_t) \
        //     union (select array_join(s, '') as s, 'd1' as g from diag1) \
        //     union (select array_join(s, '') as s, 'd2' as g from diag2) \
        //     "
            
        // .to_string(),
        // "create view text_all as select s, g, 'f' as r from text_fwd union (select reverse(s), g, 't' as r from text_fwd) order by g, r"
        //     .to_string(),

        // "create view matches as select regexp_count(s, 'XMAS') as c, s from text_all".to_string()
    ];

    // for def in view_defs {
    //     ctx.sql(&def).await.unwrap().collect().await.unwrap();
    // }

    ctx.sql("create view rows_tmp as select unnest(s) as s, row_number() OVER (PARTITION BY 'a') - 1 as y from grid")
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();

        ctx.sql(&format!("create view grid_coords as select distinct s, y, (row_number() OVER (PARTITION BY 'a') - 1) % {} as x from rows_tmp order by y", grid.len()))
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();

    let mut total = 0;
    
    for (x_mod, y_mod) in vec![
        (" + 1", ""),
        (" - 1", ""),
        ("", " + 1"),
        ("", " - 1"),
        (" + 1", " + 1"),
        (" + 1", " - 1"),
        (" - 1", " + 1"),
        (" - 1", " - 1"),
    ] {
        let uber_query = format!("
            select g1.y, g1.x from grid_coords g1
                inner join grid_coords g2 on g2.x = g1.x {} and g2.y = g1.y {}
                inner join grid_coords g3 on g3.x = g2.x {} and g3.y = g2.y {}
                inner join grid_coords g4 on g4.x = g3.x {} and g4.y = g3.y {}
                where
                    g1.s = 'X' and
                    g2.s = 'M' and
                    g3.s = 'A' and
                    g4.s = 'S'
            order by g1.y, g1.x
        ", x_mod, y_mod, x_mod, y_mod, x_mod, y_mod);
        let result = ctx
        .sql(&uber_query)
        .await
        .unwrap()
        .count()
        .await
        .unwrap();
        total += result

    }




    println!("{}", total);
}
