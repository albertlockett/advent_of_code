//   
// 1010111   
// ???.### 1,1,3

//  1      1 111_
//       1 1 111_
// .??..??...?##. 1,1,3
//  0      0 111_

fn main() {
    let masks = vec![1, 1, 3];
    let mask_rs = v_to_rs(masks.clone()); // todo avoid clone?

    let p_mask = 0b000000000011;
    let n_mask = 0b110011000111;
    let mask_bits = 12;

    let num_gaps = (masks.len() - 1) as u8;
    let gap_bits = mask_bits - masks.iter().sum::<u8>();
    let gaps = compute_gaps(num_gaps, gap_bits);
    // let gaps = vec![
    //     vec![1, 1], vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1], vec![6, 1],
    //     vec![1, 2], vec![2, 2], vec![3, 2], vec![4, 2], vec![5, 2],
    //     vec![1, 3], vec![2, 3], vec![3, 3], vec![4, 3],
    //     vec![1, 4], vec![2, 4], vec![3, 4],
    //     vec![1, 5], vec![2, 5],
    //     vec![1, 6]
    // ];

    let mut results = 0;
    for gap in gaps {
        let mut t = compute_t(&masks, &gap);
        t |= p_mask;
        t &= n_mask;
        let t_rs = to_rs(t);
        if t_rs == mask_rs {
            println!("found match: {}", format!("{:#b}", t));
            results += 1;
        }
    }

    println!("results: {}", results);


            //     ??..??...?##
    // let p_mask = 0b000000000011;
    // let n_mask = 0b110011000111;

    // let mut t_mask = 0;
    // t_mask = t_mask + masks[0];
    // t_mask <<= 2;
    // t_mask = t_mask + masks[1];
    // t_mask <<= 4;
    // t_mask = t_mask + masks[2];

    // let mut test = t_mask | p_mask;
    // test &= n_mask;

    // check if test is 113

    // find each combination of spaces can have between masks
    // 1 bit, 1 bit, 3 bits
    // where total length is 12 bits
    // 12 - 1 - 1 - 3 = 7
    // there's 2 spaces in between msaks
    // so how many permutations of the digits 1 -> 6 can we make
    // 1,1 2,1 3,1 4,1 5,1 6,1
    // 1,2 2,2 3,2 4,2 5,2
    // 1,3 2,3 3,3 4,3
    // 1,4 2,4 3,4
    // 1,5 2,5
    // 1,6
    // ....
    // 6 + 5 + 4 + 3 + 2 + 1 = 21

    // for each combo above, create our mask and slide it from left to right
    // calculate test and check if the resut is 113

    println!("Hello, world!");
}

fn compute_gaps(num_gaps: u8, gbits: u8) -> Vec<Vec<u8>> {
    if num_gaps == 1 {
        let mut results = vec![];
        for i in 0..gbits {
            results.push(vec![i + 1]);
        }
        return results
    }

    let l_gaps = compute_gaps(num_gaps - 1, gbits - 1);
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

fn compute_t(masks: &Vec<u8>, gaps: &Vec<u8>) -> u32 {
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

fn v_to_rs(t: Vec<u8>) -> u32 {
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

fn to_rs(t: u32) -> u32 {
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
    fn test_compute_gaps() {
        let result = compute_gaps(2, 7);
        let expected = vec![
            vec![1, 1], vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1], vec![6, 1],
            vec![1, 2], vec![2, 2], vec![3, 2], vec![4, 2], vec![5, 2],
            vec![1, 3], vec![2, 3], vec![3, 3], vec![4, 3],
            vec![1, 4], vec![2, 4], vec![3, 4],
            vec![1, 5], vec![2, 5],
            vec![1, 6]
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
