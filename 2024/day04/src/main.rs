use std::sync::Arc;

use arrow_array::{
    builder::{ListBuilder, StringBuilder},
    RecordBatch,
};
use arrow_schema::{DataType, Field, Schema};
use datafusion::{
    arrow::util::pretty::print_batches, common::utils::transpose, prelude::SessionContext,
};

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
    let dims = grid[0].len();

    let mut list_builder = ListBuilder::new(StringBuilder::new());
    grid.iter()
        .for_each(|row| list_builder.append_value(row.iter().map(Some).collect::<Vec<_>>()));
    let batch =
        RecordBatch::try_new(schema.clone(), vec![Arc::new(list_builder.finish())]).unwrap();
    ctx.register_batch("grid", batch).unwrap();

    grid.iter()
        .for_each(|row| list_builder.append_value(row.iter().rev().map(Some).collect::<Vec<_>>()));
    let batch =
        RecordBatch::try_new(schema.clone(), vec![Arc::new(list_builder.finish())]).unwrap();
    ctx.register_batch("grid_rev", batch).unwrap();

    let grid = transpose(grid.clone());
    grid.iter()
        .for_each(|row| list_builder.append_value(row.iter().map(Some).collect::<Vec<_>>()));
    let batch =
        RecordBatch::try_new(schema.clone(), vec![Arc::new(list_builder.finish())]).unwrap();
    ctx.register_batch("grid_t", batch).unwrap();

    let tmp_view_def = "select unnest(s) as s, row_number() OVER (PARTITION BY 'a') - 1 as y";
    let coords_view_def = format!(
        "select distinct s, y, (row_number() OVER (PARTITION BY 'a') - 1) % {} as x",
        dims
    );
    let view_defs = vec![
        format!("create view rows_tmp as {} from grid", tmp_view_def),
        format!("create view rows_tmp_rev as {} from grid_rev", tmp_view_def),
        format!(
        "create view grid_coords as {} from rows_tmp order by y",
            coords_view_def
        ),
        format!(
            "create view grid_coords_rev as {} from rows_tmp_rev order by y",
                coords_view_def
            ),
        "create view diag as select array_agg(s order by y asc) as s, x + y as sum from grid_coords group by x + y order by x + y".to_string(),
        "create view diag_rev as select array_agg(s order by y asc) as s, x + y as sum from grid_coords_rev group by x + y order by x + y".to_string(),
        "create view text_fwd as \
            select array_join(s, '') as s, 'g1' as g from grid \
            union (select array_join(s, '') as s, 'gt' as g from grid_t) \
            union (select array_join(s, '') as s, 'd1' as g from diag) \
            union (select array_join(s, '') as s, 'd2' as g from diag_rev) \
            "
        .to_string(),
        "create view text_all as select s, g, 'f' as r from text_fwd union (select reverse(s), g, 't' as r from text_fwd) order by g, r"
            .to_string(),

        "create view matches as select regexp_count(s, 'XMAS') as c, s from text_all".to_string()
    ];

    for def in view_defs {
        ctx.sql(&def).await.unwrap().collect().await.unwrap();
    }
    println!("part 1 not working");
    let result = ctx
        .sql("select sum(c) as part1 from matches")
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();
    print_batches(&result).unwrap();

    // part 2
    let offset_cords = vec![
        ((2, 0), (1, 1), (0, 2), (2, 2)),
        ((0, 2), (-1, 1), (-2, 0), (-2, 2)),
        ((2, 0), (1, -1), (0, -2), (2, -2)),
        ((0, 2), (1, 1), (2, 0), (2, 2)),
    ];

    let mut p2_total = 0;
    for ((x1, y1), (x2, y2), (x3, y3), (x4, y4)) in offset_cords.iter() {
        let uber_query = format!(
            "select g1.y, g1.x from grid_coords g1
                inner join grid_coords g2 on g2.x = g1.x + {} and g2.y = g1.y + {}
                inner join grid_coords g3 on g3.x = g1.x + {} and g3.y = g1.y + {}
                inner join grid_coords g4 on g4.x = g1.x + {} and g4.y = g1.y + {}
                inner join grid_coords g5 on g5.x = g1.x + {} and g5.y = g1.y + {}
                where
                    g1.s = 'M' and
                    g2.s = 'M' and
                    g3.s = 'A' and
                    g4.s = 'S' and
                    g5.s = 'S'
                    
            ",
            x1, y1, x2, y2, x3, y3, x4, y4
        );
        let result = ctx.sql(&uber_query).await.unwrap().count().await.unwrap();
        p2_total += result
    }

    println!("p2 = {}", p2_total);
}
