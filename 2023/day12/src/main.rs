use std::fs::File;
use std::io::prelude::*;
use std::num::NonZeroUsize;

//
// 1010111
// ???.### 1,1,3

//  1      1 111_
//       1 1 111_
// .??..??...?##. 1,1,3
//  0      0 111_

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();
    let parsed_lines = lines
        .map(|line| ParsedLine::new(line, true))
        .collect::<Vec<ParsedLine>>();

    let mut total_result = 0;
    for pl in parsed_lines {
        total_result += pl.compute_arrs();

    }
    println!("p1 total {}", total_result);
}

struct ParsedLine {
    line_raw: String,
    seq: Vec<u8>,
    check_seq_rs: u128,

    p_mask: u128,
    n_mask: u128,
    mask_bits: u8, // length of the mask in bits

    // length and offset
    contig_rs: Vec<(u8, u8)>,
}

impl ParsedLine {
    fn new(line: &str, is_part_2: bool) -> Self {

        let mut sec_iter = line.split(" ");

        // compute masks
        let mask_sec = sec_iter.next().unwrap();
        let first_non_dot_index = mask_sec.find(|c| c != '.').unwrap();
        let last_non_dot_index = mask_sec.rfind(|c| c != '.').unwrap();

        let mask_seq_raw = &mask_sec[first_non_dot_index..=last_non_dot_index];

        let mut p_mask = 0;
        let mut n_mask = 0;
        let mut mask_bits = mask_seq_raw.len() as u8;

        let mut contig_rs = vec![];
        let mut segment_len = 0;
        let mut segment_offset = 0;
        mask_seq_raw.chars().for_each(|c| {
            p_mask <<= 1;
            n_mask <<= 1;
            if c == '#' {
                p_mask += 1;
            }
            if c != '.' {
                segment_len += 1;
                n_mask += 1;
                
            } else {
                if segment_len > 0 {
                    contig_rs.push((segment_len, segment_offset - segment_len));
                }
                segment_len = 0;
            }
            segment_offset += 1;
        });
        if segment_len > 0 {
            contig_rs.push((segment_len, segment_offset - segment_len));
        }

        if is_part_2 {
            let orig_p_mask = p_mask;
            let orig_n_mask = n_mask;
            for _ in 0..5 {
                p_mask <<= mask_bits;
                p_mask |= orig_p_mask;

                n_mask <<= mask_bits;
                n_mask |= orig_n_mask;
            }
            mask_bits *= 5;
        }


        // compute sequence

        let seq_sec = sec_iter.next().unwrap();
        let mut seq = vec![];
        seq_sec.split(",").for_each(|x| {
            seq.push(x.parse::<u8>().unwrap());
        });
        if is_part_2 {
            for _ in 0..4 {
                seq_sec.split(",").for_each(|x| {
                    seq.push(x.parse::<u8>().unwrap());
                });
            }
        }
        let check_seq_rs = v_to_rs(seq.clone());

        ParsedLine {
            line_raw: line.to_string(),
            seq,
            check_seq_rs,
            p_mask,
            n_mask,
            mask_bits,
            contig_rs
        }
    }

    fn compute_arrs(&self) -> u32 {
        // let mut results = 0;
        let num_gaps = (self.seq.len() - 1) as u8;
        let gap_bits = self.mask_bits - self.seq.iter().sum::<u8>();
        let gaps = all_compute_gaps(num_gaps, gap_bits);

        let total_gaps = gaps.len();
        println!("gaps for line {} : {}", self.line_raw, total_gaps);

        let mut all_results = vec![];

        let seq_total = self.seq.iter().sum::<u8>();

        let mut cache: lru::LruCache<u128, u128> = lru::LruCache::new(NonZeroUsize::new(100000).unwrap());

        let mut i = 0;
        for gap in gaps {
            i += 1;
            if i % 100000 == 0 {
                println!("{} / {}", i, total_gaps);
            }

            let t = compute_t(&self.seq, &gap);
            let gap_total = gap.iter().sum::<u8>();
            let max_left_slide = self.mask_bits - seq_total - gap_total;
            
            for i in 0..max_left_slide + 1 {
                let mut t = t;
                t <<= i;
                t |= self.p_mask;
                t &= self.n_mask;

                let t_rs = match cache.get(&t) {
                    Some(t_rs) => *t_rs,
                    None => {
                        let t_rs = to_rs(t);
                        cache.put(t, t_rs);
                        t_rs
                    },
                };
                
                if t_rs == self.check_seq_rs {
                    if !all_results.contains(&t) {
                        print!("found result for line:\n{}\n {}\n\n", self.line_raw, format!("{:012b}", t));
                        all_results.push(t);
                    }
                }
            }            
        }
        println!("results fror line {} : {}", self.line_raw, all_results.len());

        all_results.len() as u32
    }

}

// not the good algorithm, too many gaps when brute forcing it
fn all_compute_gaps(num_gaps: u8, gbits: u8) -> Vec<Vec<u8>> {
    if num_gaps == 1 {
        let mut results = vec![];
        for i in 0..gbits {
            results.push(vec![i + 1]);
        }
        return results;
    }

    let l_gaps = all_compute_gaps(num_gaps - 1, gbits - 1);
    let mut results = vec![];

    for l_gap in l_gaps {
        let total = l_gap.iter().sum::<u8>();
        for i in 0..gbits - total {
            let mut gap = l_gap.clone();
            gap.push(i + 1);
            results.push(gap);
        }
    }

    return results;
}

fn compute_possible_ranges(masks: &Vec<u8>, contig_ts: &Vec<(u8, u8)>, mask_bits: u8, p_mask: u128, n_mask: u128, check_seq_rs: u128) {
    let mut ok_mask = 0;
    for mask in masks {

        // 1s for the number of mask bits
        let mut mask_b = 0;
        for _ in 0..*mask {
            mask_b <<= 1;
            mask_b += 1;
            ok_mask <<= 1;
            ok_mask += 1;
        }

        for contig_t in contig_ts {
            let mut range_start = None;

            let seg_len = contig_t.0;
            let seg_offset = contig_t.1;
            
            if seg_len < *mask {
                continue;
            }

            let mut mask_b = mask_b;
            // move to start of segment
            mask_b <<= (mask_bits - contig_t.1) as u128; // TODO check off by one

            let mut segment_ranges = vec![];

            for i in 0..seg_len {
                mask_b >>= 1;
                println!("{:012b} = mask_b", mask_b);
                let mut t = mask_b | p_mask;
                t &= n_mask;
                println!("{:012b} = t", t);
                let t_rs = to_rs(t);
                println!("{:012b} = t_rs", t_rs);

                println!("{:012b} = ok_mask", ok_mask);
                println!("{:012b} = check_seq_rs", check_seq_rs);

                if ok_mask & check_seq_rs == ok_mask & t_rs {
                    if range_start == None {
                        range_start = Some(i);
                    }
                } else {
                    if range_start != None {
                        // end of range
                        segment_ranges.push((range_start.unwrap(), i));
                    }
                    range_start = None;
                }
            }
            if range_start != None {
                segment_ranges.push((range_start.unwrap(), seg_len));
            }
        }

        ok_mask <<= 1;
    }

}

#[test]
fn test_part_2_scratch() {
    let line = ".??..??...?##. 1,1,3";
    let parsed_line = ParsedLine::new(line, true);
    assert_eq!(parsed_line.seq, vec![1, 1, 3]);
    assert_eq!(parsed_line.check_seq_rs, 0b1110101);
    assert_eq!(parsed_line.p_mask, 0b0000000000011);
    assert_eq!(parsed_line.n_mask, 0b0110011000111);
    assert_eq!(parsed_line.mask_bits, 12);
    assert_eq!(parsed_line.compute_arrs(), 4);
    assert_eq!(parsed_line.contig_rs, vec![(2, 0), (2, 4), (3, 9)]);

    let mask_b = 0b111;
    

}

// create the value t for testing against the thing
fn compute_t(masks: &Vec<u8>, gaps: &Vec<u8>) -> u128 {
    let mut result = 0;
    for i in 0..masks.len() {
        let mask = masks[i];
        for _ in 0..mask {
            result <<= 1;
            result += 1;
        }

        if i < masks.len() - 1 {
            let gap = gaps[i];
            for _ in 0..gap {
                result <<= 1;
            }
        }
    }

    result
}

// this converts to rs what we test if the repair sequence is correct
fn v_to_rs(t: Vec<u8>) -> u128 {
    let mut result = 0;
    t.iter().rev().for_each(|x| {
        result <<= 1;
        for _ in 0..*x {
            result <<= 1;
            result += 1;
        }
    });

    result
}

// turn the results of compute_t into something we can test output
fn to_rs(t: u128) -> u128 {
    let mut t = t;
    let mut result = 0;
    while t > 0 {
        if t & 1 == 1 {
            result <<= 1;
            result += 1;
        } else {
            if result & 1 == 1 {
                result <<= 1;
            }
        }
        t >>= 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsed_line() {
        let line = ".??..??...?##. 1,1,3";
        let parsed_line = ParsedLine::new(line, false);
        assert_eq!(parsed_line.seq, vec![1, 1, 3]);
        assert_eq!(parsed_line.check_seq_rs, 0b1110101);
        assert_eq!(parsed_line.p_mask, 0b0000000000011);
        assert_eq!(parsed_line.n_mask, 0b0110011000111);
        assert_eq!(parsed_line.mask_bits, 12);
        assert_eq!(parsed_line.compute_arrs(), 4);
        assert_eq!(parsed_line.contig_rs, vec![(2, 0), (2, 4), (3, 9)]);

        compute_possible_ranges(&parsed_line.seq, &parsed_line.contig_rs, parsed_line.mask_bits, parsed_line.p_mask, parsed_line.n_mask, parsed_line.check_seq_rs)
    }


    #[test]
    fn test_parsed_line2() {
        let line = "????.######..#####. 1,6,5";
        let parsed_line = ParsedLine::new(line, false);
        assert_eq!(parsed_line.seq, vec![1, 6, 5]);
        assert_eq!(parsed_line.check_seq_rs, 0b11111011111101);
        assert_eq!(parsed_line.p_mask, 0b1111110011111);
        assert_eq!(parsed_line.n_mask, 0b111101111110011111);
        assert_eq!(parsed_line.mask_bits, 18);
        assert_eq!(parsed_line.compute_arrs(), 4);
    }

    #[test]
    fn test_parsed_line3() {
        let line = "?###???????? 3,2,1";
        let parsed_line = ParsedLine::new(line, false);
        assert_eq!(parsed_line.compute_arrs(), 10);
    }

    #[test]
    fn test_compute_gaps() {
        let result = all_compute_gaps(2, 7);
        let expected = vec![
            vec![1, 1],
            vec![2, 1],
            vec![3, 1],
            vec![4, 1],
            vec![5, 1],
            vec![6, 1],
            vec![1, 2],
            vec![2, 2],
            vec![3, 2],
            vec![4, 2],
            vec![5, 2],
            vec![1, 3],
            vec![2, 3],
            vec![3, 3],
            vec![4, 3],
            vec![1, 4],
            vec![2, 4],
            vec![3, 4],
            vec![1, 5],
            vec![2, 5],
            vec![1, 6],
        ];
        for e in expected {
            assert_eq!(true, result.contains(&e));
        }
    }

    #[test]
    fn test_compute_t() {
        let masks = vec![1, 1, 3];
        let gaps = vec![1, 2];
        assert_eq!(compute_t(&masks, &gaps), 0b10100111);

        let masks = vec![1, 1, 3];
        let gaps = vec![2, 1];
        assert_eq!(compute_t(&masks, &gaps), 0b10010111);
    }

    #[test]
    fn test_to_rs() {
        assert_eq!(to_rs(0b1001), (0b101)); // eliminates double spaces
        assert_eq!(to_rs(0b1010), (0b101)); // trailing 0
        assert_eq!(to_rs(0b1001100111), (0b11101101)); // seq len 3
        assert_eq!(to_rs(0b01000110011100111100), 0b1111011101101);
    }

    #[test]
    fn test_v_to_rs() {
        assert_eq!(v_to_rs(vec![1, 1, 3]), 0b1110101);
    }
}
