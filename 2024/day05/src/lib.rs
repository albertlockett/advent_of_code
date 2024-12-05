// coordinates for valid rule bitmask
#[inline]
fn mask_coords(l: u8, r: u8) -> (usize, u8) {
    let offset = r / 8;
    let bit = r % 8;
    let idx = l as usize * 13 + offset as usize;

    (idx, bit)
}

// convert chars to page number
#[inline]
fn to_page_num(input: &[u8], i: usize) -> u8 {
    (input[i] - 48) * 10 + (input[i + 1] - 48)
}

#[derive(PartialEq)]
enum Valid {
    Yes,
    // these are indices in an update that are invalid
    No(usize, usize),
}

// check if update is valid
#[inline]
fn is_valid(update: &mut [u8], masks: &[u8]) -> Valid {
    for l_idx in 0..update.len() - 1 {
        for r_idx in l_idx + 1..update.len() {
            let l = update[l_idx];
            let r = update[r_idx];

            let (idx, bit) = mask_coords(l, r);
            if masks[idx] & 1 << bit == 0 {
                return Valid::No(l_idx, r_idx);
            }
        }
    }

    Valid::Yes
}

#[inline]
fn rearrange_until_valid(update: &mut [u8], masks: &[u8]) {
    while let Valid::No(l_idx, r_idx) = is_valid(update, masks) {
        update.swap(l_idx, r_idx);
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

    let mut masks = vec![0u8; 100 * 13];

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
            if is_valid(&mut update, &masks) == Valid::Yes {
                p1_total += mid(&mut update) as u32;
            } else {
                rearrange_until_valid(&mut update, &masks);
                p2_total += mid(&mut update) as u32;
            }

            update.clear();
        }

        if i + 1 > input_p2.len() {
            break;
        }
    }

    (p1_total, p2_total)
}
