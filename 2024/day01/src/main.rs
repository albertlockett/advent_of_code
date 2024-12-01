
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use aoc::core::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let file = File::open("inputs/day01/test.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("{:?}", line)
    }

    Ok(())
}
