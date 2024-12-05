const MASK_BYTES_PER_PAGE: usize = 32;
const MAX_INPUT: usize = 192;

// coordinates for valid rule bitmask
#[inline]
fn mask_coords(l: u8, r: u8) -> (usize, u8) {
    let offset = r >> 3;
    let bit = r % 8;

    let idx = ((l as usize) << 5) + offset as usize;

    (idx, bit)
}

// convert chars to page number
#[inline]
fn to_page_num(input: &[u8], i: usize) -> u8 {
    ((input[i] & 0b1111) << 4) | ((input[i + 1]) & 0b1111)
}

#[inline]
fn to_real(page_num: u8) -> u8 {
    (page_num >> 4) * 10 + (page_num & 0b1111)
}

// check if update is valid
#[inline]
fn is_valid(update: &mut [u8], masks: &[u8]) -> bool {
    for l_idx in 0..update.len() - 1 {
        let l = update[l_idx];
        let r = update[l_idx + 1];

        let (idx, bit) = mask_coords(l, r);
        if masks[idx] & 1 << bit == 0 {
            return false;
        }
    }

    true
}

#[inline]
fn rearrange_until_valid(update: &mut [u8], masks: &[u8]) {
    let mut valid = false;
    while !valid {
        valid = true;
        // only needs to be valid up until the mid point
        for l_idx in 0..update.len() / 2 + 1 {
            for r_idx in l_idx + 1..update.len() {
                let l = update[l_idx];
                let r = update[r_idx];

                let (idx, bit) = mask_coords(l, r);
                if masks[idx] & 1 << bit == 0 {
                    update.swap(l_idx, r_idx);
                    valid = false;
                }
            }
        }
    }
}

#[inline]
fn mid(update: &mut [u8]) -> u8 {
    *update.get(update.len() / 2).unwrap()
}

#[inline]
pub fn doit() -> (u32, u32) {
    let input_p1 = include_bytes!("../../inputs/day05/real_p1.txt");
    let input_p2 = include_bytes!("../../inputs/day05/real_p2.txt");

    let mut masks = vec![0u8; (MAX_INPUT + 2) * MASK_BYTES_PER_PAGE];

    let mut i = 0;
    while i < input_p1.len() {
        let l = to_page_num(input_p1, i);
        let r = to_page_num(input_p1, i + 3);
        let (idx, bit) = mask_coords(l, r);

        masks[idx] |= 1 << bit;
        i += 6;
    }

    let mut i = 0;
    let mut update: Vec<u8> = vec![];
    let mut p1_total: u32 = 0;
    let mut p2_total: u32 = 0;

    loop {
        let l = to_page_num(input_p2, i);
        i += 2;
        update.push(l);

        if i >= input_p2.len() {
            break;
        }
        let c = input_p2[i];
        i += 1;
        if c == b'\n' {
            if is_valid(&mut update, &masks) {
                p1_total += to_real(mid(&mut update)) as u32;
            } else {
                rearrange_until_valid(&mut update, &masks);
                p2_total += to_real(mid(&mut update)) as u32;
            }

            update.clear();
        }

        if i + 1 > input_p2.len() {
            break;
        }
    }

    (p1_total, p2_total)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_num() {
        for i in 0..10 {
            let n = format!("0{}", i);
            let bytes = n.as_bytes();
            let pn = to_page_num(&bytes, 0);
            let real = to_real(pn);
            println!("n = {} = {} ({:b}) = {}", n, pn, pn, real);
            assert_eq!(real, i)
        }

        for i in 10..100 {
            let n = format!("{}", i);
            let bytes = n.as_bytes();
            let pn = to_page_num(&bytes, 0);
            let real = to_real(pn);
            println!("n = {} = {} ({:b}) = {}", n, pn, pn, real);
            assert_eq!(real, i)
        }
    }
}
