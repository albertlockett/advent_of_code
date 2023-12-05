use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, Sub};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut raw_input_iter = contents.split("\n\n").into_iter();
    let seeds = parse_seeds(raw_input_iter.next().unwrap());

    let category_maps = raw_input_iter
        .map(|segment| CategoryMap::new(segment))
        .collect::<Vec<CategoryMap<u64>>>();

    let mut final_dests = vec![];
    for seed in seeds {
        let mut src = seed;
        for category_map in &category_maps {
            for range in &category_map.ranges {
                if range.contains_src(src) {
                    src = range.to_dst(src);
                    break;
                }
            }
        }
        final_dests.push(src);
    };

    let result = final_dests.into_iter().min().unwrap();
    println!("results p1: {:?}", result);

    Ok(())
}

struct Range<T> {
    dst_start: T,
    src_start: T,
    length: T,
}

impl<T: Add<Output = T> + FromStr + PartialOrd + Copy + Sub<Output = T>> Range<T>
where
    <T as FromStr>::Err: Debug,
{
    fn new(line: &str) -> Self {
        let mut iter = line.split(" ").into_iter();

        let dst_start = iter.next().unwrap().parse::<T>().unwrap();
        let src_start = iter.next().unwrap().parse::<T>().unwrap();
        let length = iter.next().unwrap().parse::<T>().unwrap();

        Range {
            dst_start,
            src_start,
            length,
        }
    }

    fn contains_src(&self, src: T) -> bool {
        src >= self.src_start && src < (self.src_start + self.length)
    }

    fn to_dst(&self, src: T) -> T {
        src - self.src_start + self.dst_start
    }
}

#[test]
fn test_range() {
    let input = "50 98 2";
    let range: Range<u32> = Range::new(input);
    assert_eq!(range.dst_start, 50);
    assert_eq!(range.src_start, 98);
    assert_eq!(range.length, 2);

    assert_eq!(range.contains_src(49), false);
    assert_eq!(range.contains_src(98), true);
    assert_eq!(range.contains_src(99), true);
    assert_eq!(range.contains_src(100), false);

    assert_eq!(range.to_dst(98), 50);
    assert_eq!(range.to_dst(99), 51);
}

struct CategoryMap<T> {
    src: String,
    dst: String,
    ranges: Vec<Range<T>>,
}

impl<T: Add<Output = T> + FromStr + PartialOrd + Copy + Sub<Output = T>> CategoryMap<T>
where
    <T as FromStr>::Err: Debug,
{
    fn new(segment: &str) -> Self {
        let mut lines_iter = segment.split("\n").into_iter();
        let mut meta_iter = lines_iter
            .next()
            .unwrap()
            .split(" ")
            .into_iter()
            .next()
            .unwrap()
            .split("-")
            .into_iter();
        let src = meta_iter.next().unwrap().to_string();
        meta_iter.next();
        let dst = meta_iter.next().unwrap().to_string();

        let ranges = lines_iter
            .map(|line| Range::new(line.trim()))
            .collect::<Vec<Range<T>>>();

        CategoryMap { src, dst, ranges }
    }
}

#[test]
fn test_new_category_map() {
    let input = "seed-to-soil map:
    50 98 2
    52 50 48";
    let category_map: CategoryMap<u32> = CategoryMap::new(input);
    assert_eq!(category_map.src, "seed");
    assert_eq!(category_map.dst, "soil");
    assert_eq!(category_map.ranges.len(), 2);
}

fn parse_seeds<T>(raw: &str) -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug {
    let nums_raw = raw.split(":").into_iter().skip(1).next().unwrap().trim();
    let result = nums_raw
        .split(" ")
        .into_iter()
        .map(|n| n.parse::<T>().unwrap())
        .collect();

    result
}

#[test]
fn test_parse_seeds() {
    let input = "seeds: 79 14 55 13";
    let result = parse_seeds(input);
    assert_eq!(vec![79, 14, 55, 13], result);
}
