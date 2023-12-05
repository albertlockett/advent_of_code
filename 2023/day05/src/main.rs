use std::cmp::PartialOrd;
use std::collections::HashSet;
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
    }

    let result = final_dests.into_iter().min().unwrap();
    println!("results p1: {:?}", result);

    // part 2 ...
    let seeds = parse_seeds_as_ranges(seeds_raw);
    let mut final_dests = vec![];

    for seed_range in seeds {
        let mut src_ranges = vec![seed_range];

        // iterate thru each category map
        for category_map in &category_maps {
            let mut dst_ranges = vec![]; // these are destinations we've mapped

            // iterate through the ranges
            for range in &category_map.ranges {
                // this will hold seed ranges that weren't mapped
                let mut nxt_src_ranges = vec![];

                for src_range in src_ranges {
                    let (in_range, out_range, outer_range_2) =
                        range.to_dst_range(src_range.0, src_range.1);
                    if in_range.is_some() {
                        dst_ranges.push(in_range.unwrap());
                    }

                    // if there's a range that wasn't mapped, we'll need to keep it
                    // for the next range
                    if out_range.is_some() {
                        nxt_src_ranges.push(out_range.unwrap());
                    }
                    if outer_range_2.is_some() {
                        nxt_src_ranges.push(outer_range_2.unwrap());
                    }
                }

                src_ranges = nxt_src_ranges;
            }

            // dedupe the ranges (maybe not necessary)
            dst_ranges = dedupe_ranges(dst_ranges);

            // pass any unmapped ranges + mapped ranges to next category
            src_ranges.extend(dst_ranges);
        }

        final_dests.push(src_ranges)
    }

    // scan results to find min
    let result = final_dests
        .iter()
        .map(|ranges| ranges.into_iter().map(|(start, _)| start).min().unwrap())
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

    // will return 3 ranges:
    // mapped range for sectino of argument range that overlaps with current rnage
    // possibly 2 other ranges for parts of the argument range that don't overlap
    // this source range
    fn to_dst_range(
        &self,
        start: T,
        length: T,
    ) -> (Option<(T, T)>, Option<(T, T)>, Option<(T, T)>) {
        if start < self.src_start {
            let end = start + length;

            // range starts lower than self range and doesn't overlap
            if end <= self.src_start {
                return (None, Some((start, length)), None);
            }

            let lower_range = (start, self.src_start - start);

            // range extends both sides of self range
            if end > self.src_start + self.length {
                let middle_range = (self.to_dst(self.src_start), self.length);

                let upper_range = (
                    self.src_start + self.length,
                    end - self.src_start - self.length,
                );

                return (Some(middle_range), Some(lower_range), Some(upper_range));
            }

            // range starts lower and overlaps with self range
            let upper_range = (self.to_dst(self.src_start), length - lower_range.1);

            return (Some(upper_range), Some(lower_range), None);
        } else if start + length <= self.src_start + self.length {
            // range is contained by self range
            let lower_range = (self.to_dst(start), length);

            return (Some(lower_range), None, None);
        } else {
            let self_end = self.src_start + self.length;

            // range starts greater than self range start and doesn't overlap
            if start >= self_end {
                return (None, Some((start, length)), None);
            }

            // range starts greater than self range start and overlaps
            let lower_range = (self.to_dst(start), self_end - start);
            let upper_range = (self_end, length - lower_range.1);
            return (Some(lower_range), Some(upper_range), None);
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

    let input = "20 10 5";
    let range: Range<u32> = Range::new(input);
    assert_eq!(range.to_dst_range(9, 1), (None, Some((9, 1)), None));
    assert_eq!(range.to_dst_range(10, 1), (Some((20, 1)), None, None));
    assert_eq!(range.to_dst_range(10, 4), (Some((20, 4)), None, None));
    assert_eq!(range.to_dst_range(11, 3), (Some((21, 3)), None, None));
    assert_eq!(
        range.to_dst_range(8, 4),
        (Some((20, 2)), Some((8, 2)), None)
    );
    assert_eq!(
        range.to_dst_range(13, 6),
        (Some((23, 2)), Some((15, 4)), None)
    );
    assert_eq!(range.to_dst_range(14, 1), (Some((24, 1)), None, None));
    assert_eq!(
        range.to_dst_range(14, 2),
        (Some((24, 1)), Some((15, 1)), None)
    );
    assert_eq!(range.to_dst_range(16, 2), (None, Some((16, 2)), None));
    assert_eq!(range.to_dst_range(15, 2), (None, Some((15, 2)), None));

    assert_eq!(
        range.to_dst_range(9, 20),
        (Some((20, 5)), Some((9, 1)), Some((15, 14)))
    );
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

fn parse_seeds<T>(raw: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
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

fn parse_seeds_as_ranges<T>(raw: &str) -> Vec<(T, T)>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let nums_raw = raw.split(":").into_iter().skip(1).next().unwrap().trim();
    let mut nums_iter = nums_raw.split(" ").into_iter().peekable();

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
    let mut contains: HashSet<String> = HashSet::new();
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
