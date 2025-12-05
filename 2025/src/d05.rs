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

    fn do_p2(&mut self, input: &str) -> Result<usize> {
        let db = FoodDb::try_from_byte_iter(Self::read_input_iter(input)?.bytes())?;
        let mut count_fresh_ids = 0;

        for i in 0..db.ranges.len() {
            let mut curr_ranges = vec![db.ranges[i]];

            // backtrack through previous ranges and remove over-lappings from start and end
            for j in 0..i {
                let mut new_ranges = vec![];
                let prev_range = db.ranges[j];

                #[allow(clippy::needless_range_loop)]
                for k in 0..curr_ranges.len() {
                    let chopped = chop_range(curr_ranges[k], prev_range);
                    let split = split_range(chopped, prev_range);
                    curr_ranges[k] = split.0;
                    if let Some(split_rhs) = split.1 {
                        new_ranges.push(split_rhs);
                    }
                }
                curr_ranges.append(&mut new_ranges);
            }

            for curr_range in curr_ranges {
                if curr_range != (0, 0) {
                    count_fresh_ids += curr_range.1 - curr_range.0 + 1;
                }
            }
        }

        Ok(count_fresh_ids)
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

fn chop_range(target: (usize, usize), to_chop: (usize, usize)) -> (usize, usize) {
    // check full overlap
    if to_chop.0 <= target.0 && to_chop.1 >= target.1 {
        return (0, 0);
    }

    // check overlap at start
    if to_chop.0 <= target.0 && to_chop.1 <= target.1 && to_chop.1 >= target.0 {
        return (to_chop.1 + 1, target.1);
    }

    if to_chop.0 >= target.0 && to_chop.1 >= target.1 && to_chop.0 <= target.1 {
        return (target.0, to_chop.0 - 1);
    }

    target
}

fn split_range(
    target: (usize, usize),
    to_split: (usize, usize),
) -> ((usize, usize), Option<(usize, usize)>) {
    if to_split.0 > target.0 && to_split.1 < target.1 {
        ((target.0, to_split.0 - 1), Some((to_split.1 + 1, target.1)))
    } else {
        (target, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chop_range() {
        // test chops at start
        assert_eq!(chop_range((5, 10), (4, 6)), (7, 10));

        // test chops at start, start fence post
        assert_eq!(chop_range((5, 10), (5, 7)), (8, 10));

        // test chops at start, end fence post
        assert_eq!(chop_range((5, 10), (4, 5)), (6, 10));

        // test full overlap
        assert_eq!(chop_range((5, 7), (4, 8)), (0, 0));

        // test full start fence post
        assert_eq!(chop_range((5, 7), (5, 8)), (0, 0));

        // test full end fence post
        assert_eq!(chop_range((5, 7), (4, 7)), (0, 0));

        // test chops at end
        assert_eq!(chop_range((3, 8), (6, 9)), (3, 5));

        // test chops at end, start fence post
        assert_eq!(chop_range((3, 8), (8, 9)), (3, 7));

        // test chops at end, end fence post
        assert_eq!(chop_range((3, 8), (6, 8)), (3, 5));

        // test no overlap before
        assert_eq!(chop_range((1, 3), (5, 8)), (1, 3));

        // test no overlap after
        assert_eq!(chop_range((5, 6), (1, 3)), (5, 6));

        // ranges equal
        assert_eq!(chop_range((5, 8), (5, 8)), (0, 0));

        // test range in middle
        assert_eq!(chop_range((5, 10), (6, 8)), (0, 0))
    }

    #[test]
    fn test_split_ranges() {
        // test split
        assert_eq!(split_range((5, 10), (6, 7)), ((5, 5), Some((8, 10))));
    }
}
