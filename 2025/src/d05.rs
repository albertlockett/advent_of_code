use std::io::{Read, Result};

use crate::Challenge;

#[derive(Default)]
pub struct Day05 {}

impl Challenge for Day05 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let db = FoodDb::try_from_byte_iter(Self::read_input_iter(input)?.bytes())?;
        let mut count_fresh = 0;

        for id in &db.ids {
            for (start, end) in &db.ranges {
                if id >= start && id <= end {
                    count_fresh += 1;
                    break;
                }
            }
        }

        Ok(count_fresh)
    }

    fn do_p2(&mut self, _input: &str) -> Result<usize> {
        Ok(0)
    }
}

pub struct FoodDb {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

impl FoodDb {
    fn try_from_byte_iter<I: Iterator<Item = Result<u8>>>(mut iter: I) -> Result<Self> {
        let ranges = Self::try_parse_ranges(&mut iter)?;
        let ids = Self::try_parse_ids(&mut iter)?;

        Ok(Self { ranges, ids })
    }

    fn try_parse_ranges<I: Iterator<Item = Result<u8>>>(
        iter: &mut I,
    ) -> Result<Vec<(usize, usize)>> {
        let mut ranges = Vec::new();
        // parse ranges
        loop {
            let mut range_start = 0;
            let mut range_end = 0;

            // parse left range
            loop {
                let b = iter.next().expect("iter shouldnt end before ranges")?;
                match b {
                    b'\n' => return Ok(ranges),
                    b'-' => break,
                    num => {
                        range_start *= 10;
                        range_start += (num - b'0') as usize
                    }
                }
            }

            // parse right range
            loop {
                let b = iter.next().expect("iter shouldn't end before ranges")?;
                match b {
                    b'\n' => break,
                    num => {
                        range_end *= 10;
                        range_end += (num - b'0') as usize;
                    }
                }
            }

            ranges.push((range_start, range_end))
        }
    }

    fn try_parse_ids<I: Iterator<Item = Result<u8>>>(iter: &mut I) -> Result<Vec<usize>> {
        let mut food_ids = Vec::new();
        let mut food_id = 0;
        loop {
            let b = iter.next();
            match b {
                None => {
                    food_ids.push(food_id);
                    break;
                }
                Some(b) => match b? {
                    b'\n' => {
                        food_ids.push(food_id);
                        food_id = 0;
                    }
                    num => {
                        food_id *= 10;
                        food_id += (num - b'0') as usize;
                    }
                },
            }
        }

        Ok(food_ids)
    }
}
