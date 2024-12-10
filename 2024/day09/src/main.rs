#[derive(Debug, Clone)]
pub enum Segment {
    Full(u16, u8),
    Empty(u8),
}

fn main() {
    let input = include_bytes!("../../inputs/day09/real.txt");
    let input = input.iter().map(|b| b - 48).collect::<Vec<_>>();
    let input2 = input.clone();

    // part 1

    let mut i = 0;
    let mut j = input.len() - 1;
    let mut curr_file = 0;
    let mut end_file = input.len() / 2;
    let mut end_file_b = *input.last().unwrap();

    let mut end_found = false;
    let mut compacted = vec![];
    while i <= j && !end_found {
        if i == j {
            for _ in 0..end_file_b {
                compacted.push(end_file);
            }
            break;
        }
        let file_b = *input.get(i).unwrap();
        for _ in 0..file_b {
            compacted.push(curr_file);
        }
        i += 1;
        if i > j {
            break;
        }

        let free_b = *input.get(i).unwrap();
        for _ in 0..free_b {
            while end_file_b == 0 {
                end_file -= 1;
                j -= 2;
                end_file_b = *input.get(j).unwrap();

                if j <= i {
                    end_found = true;
                    break;
                }
            }
            if end_found {
                break;
            }
            compacted.push(end_file);
            end_file_b -= 1;
        }
        i += 1;
        curr_file += 1;
    }

    let p1 = compacted
        .into_iter()
        .enumerate()
        .fold(0u64, |acc, (i, f_n)| acc + (i * f_n) as u64);
    println!("p1 = {}", p1);

    // part 2

    let mut segments: Vec<Segment> = Vec::new();

    let mut curr_file = 0;
    let mut i = 0;
    while i < input2.len() {
        let file_b = *input.get(i).unwrap();
        segments.push(Segment::Full(curr_file, file_b));
        curr_file += 1;
        i += 1;
        if i >= input.len() {
            break;
        }
        let free_b = *input.get(i).unwrap();
        segments.push(Segment::Empty(free_b));
        i += 1;
    }

    let mut j = segments.len() - 1;
    while j > 0 {
        let segment_src = segments.get(j).unwrap().clone();
        match segment_src {
            Segment::Empty(_) => {}
            Segment::Full(_, src_size) => {
                let mut i = 0;
                while i < j {
                    let segment_dst = segments.get(i).unwrap().clone();
                    match segment_dst {
                        Segment::Empty(dst_size) => {
                            if dst_size >= src_size {
                                segments[i] = segment_src.clone();
                                segments[j] = Segment::Empty(src_size);
                                if dst_size > src_size {
                                    let (l, r) = segments.split_at(i + 1);
                                    let mut l = l.to_vec();
                                    let mut r = r.to_vec();
                                    segments = vec![];
                                    segments.append(&mut l);
                                    segments.push(Segment::Empty(dst_size - src_size));
                                    segments.append(&mut r);
                                    j += 1;
                                }
                                break;
                            }
                        }
                        Segment::Full(_, _) => {}
                    }
                    i += 1;
                }
            }
        }
        j -= 1;
    }

    let mut idx = 0;
    let mut p2_total: u64 = 0;
    for seg in segments.clone() {
        match seg {
            Segment::Empty(i) => {
                for _ in 0..i {
                    idx += 1;
                }
            }
            Segment::Full(f_id, i) => {
                for _ in 0..i {
                    p2_total += f_id as u64 * idx;
                    idx += 1;
                }
            }
        }
    }

    println!("\np2 = {}", p2_total)
}
