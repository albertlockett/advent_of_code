use std::cmp::PartialOrd;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, Sub};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input_test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut raw_input_iter = contents.split("\n\n").into_iter();
    let seeds_raw = raw_input_iter.next().unwrap();

    let category_maps = raw_input_iter
        .map(|segment| CategoryMap::new(segment))
        .collect::<Vec<CategoryMap<u64>>>();

    // part 1 ... 
    let seeds = parse_seeds(seeds_raw);
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


    // part 2 ...
    let seeds = parse_seeds_as_ranges(seeds_raw);
    let mut final_dests = vec![];
    for seed_range in seeds {
        let mut src_ranges = vec![seed_range];
        for category_map in &category_maps {
            let mut dst_ranges = vec![];
            for range in &category_map.ranges {
                for (range_start, range_len) in &src_ranges {
                    let (range_1, range_2) = range.to_dst_range(*range_start, *range_len);
                    dst_ranges.push(range_1);
                    if range_2.is_some() {
                        dst_ranges.push(range_2.unwrap());
                    }
                }
            }
            if dst_ranges.len() == 0 {
                break;
            }
            dst_ranges = dedupe_ranges(dst_ranges);
            src_ranges = dst_ranges;
        }
        final_dests.push(src_ranges)
    }

    let result = final_dests
        .iter()
        .map(|ranges| ranges.into_iter().map(|(start, _)| start).filter(|x| **x as u128 > 0).min().unwrap())
        .min();

    println!("results p2: {:?}", result);


    Ok(())
}

#[derive(Debug)]
struct Range<T> {
    dst_start: T,
    src_start: T,
    length: T,
}

impl<T: Add<Output = T> + FromStr + PartialOrd + Copy + Sub<Output = T> + Debug> Range<T>
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

    fn min_src_for_range(&self, start: T, length: T) -> Option<T> {
        if start >= self.src_start && start < (self.src_start + self.length) {
            Some(start)
        } else if self.src_start >= start && self.src_start < (start + length) {
            Some(self.src_start)
        } else {
            None
        }
    }

    fn max_src_for_range(&self, start: T, length: T) -> Option<T> {
        let end = start + length;
        let self_end = self.src_start + self.length;
        if end >= self.src_start && end < self_end {
            Some(end)
        } else if self_end >= start && self_end < end {
            Some(self_end)
        } else {
            None
        }
    }

    fn to_dst_range(&self, start: T, length: T) -> ((T, T), Option<(T, T)>) {
        if start < self.src_start {
            let end = start + length;
            if end <= self.src_start {
                return ((start, length), None);
            }

            let lower_range = (
                start,
                self.src_start - start
            );
            println!("lower_range: {:?}, self = {:?}, start = {:?} len = {:?}", lower_range, self, start, length);
            let upper_range = (
                self.to_dst(self.src_start),
                length - lower_range.1
            );

            return (lower_range, Some(upper_range));
        } else if start + length <= self.src_start + self.length {
            let lower_range = (
                self.to_dst(start),
                length
            );
            
            return (lower_range, None);
        } else {
            let self_end = self.src_start + self.length;
            if start >= self_end {
                return ((start, length), None);
            }
            let lower_range = (
                self.to_dst(start),
                self_end - start
            );
            let upper_range = (
                self_end,
                length - lower_range.1
            );
            return (lower_range, Some(upper_range));
        }
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

    // assert_eq!(range.min_src_for_range(49, 2), None);
    // assert_eq!(range.min_src_for_range(98, 100), Some(98));
    // assert_eq!(range.min_src_for_range(99, 100), Some(99));
    // assert_eq!(range.min_src_for_range(100, 100), None);
    // assert_eq!(range.min_src_for_range(97, 100), Some(98));

    // TODO test maxs
    // TODO test to_dst_range

    let input = "20 10 5";
    let range: Range<u32> = Range::new(input);
    // assert_eq!(range.to_dst_range(9, 1), ((9, 1), None));
    // assert_eq!(range.to_dst_range(10, 1), ((20, 1), None));
    // assert_eq!(range.to_dst_range(11, 3), ((21, 3), None));
    // assert_eq!(range.to_dst_range(8, 4), ((8, 2), Some((20, 2))));
    // assert_eq!(range.to_dst_range(13, 6), ((23, 2), Some((15, 4))));
    assert_eq!(range.to_dst_range(14, 1), ((24, 1), None));
    assert_eq!(range.to_dst_range(14, 2), ((24, 1), Some((15, 1))));
}

struct CategoryMap<T> {
    src: String,
    dst: String,
    ranges: Vec<Range<T>>,
}

impl<T: Add<Output = T> + FromStr + PartialOrd + Copy + Sub<Output = T> + Debug> CategoryMap<T>
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

fn parse_seeds_as_ranges<T>(raw: &str) -> Vec<(T, T)> where T: FromStr, <T as FromStr>::Err: Debug {
    let nums_raw = raw.split(":").into_iter().skip(1).next().unwrap().trim();
    let mut nums_iter = nums_raw
    .split(" ")
    .into_iter()
    .peekable();

    let mut result = vec![];

    while nums_iter.peek().is_some() {
        let start = nums_iter.next().unwrap().parse::<T>().unwrap();
        let end = nums_iter.next().unwrap().parse::<T>().unwrap();
        result.push((start, end));
    }
    result
}

#[test]
fn test_parse_seeds_as_ranges() {
    let input = "seeds: 79 14 55 13";
    let result = parse_seeds_as_ranges(input);
    assert_eq!(vec![(79, 14), (55, 13)], result);
}


fn dedupe_ranges<T: Debug>(ranges: Vec<(T, T)>) -> Vec<(T, T)> {
    let mut contains: HashSet<String>= HashSet::new();
    let mut results = vec![];

    for range in ranges {
        if contains.contains(format!("{:?}", range).as_str()) {
            continue;
        }
        contains.insert(format!("{:?}", range));
        results.push(range);
    }
    return results;
}