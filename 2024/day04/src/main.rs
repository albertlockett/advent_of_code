use std::sync::Arc;

use arrow_array::{
    builder::{ListBuilder, StringBuilder},
    RecordBatch,
};
use arrow_schema::{DataType, Field, Schema};
use datafusion::prelude::SessionContext;

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

    let mut p1_total = 0;

    // part 1
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
        let uber_query = format!(
            "
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
        ",
            x_mod, y_mod, x_mod, y_mod, x_mod, y_mod
        );
        let result = ctx.sql(&uber_query).await.unwrap().count().await.unwrap();
        p1_total += result
    }
    println!("p1 = {}", p1_total);

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
