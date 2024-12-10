fn main() {
    let input = include_bytes!("../../inputs/day09/real.txt");
    let input = input.iter().map(|b| b - 48).collect::<Vec<_>>();

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
            break
        }
        let file_b = *input.get(i).unwrap();
        for _ in 0..file_b {
            compacted.push(curr_file);
        }
        i += 1;
        if i > j {
            break
        }

        let free_b = *input.get(i).unwrap();
        for _ in 0..free_b {
            while end_file_b <= 0 {
                end_file -= 1;
                j -= 2;
                end_file_b = *input.get(j).unwrap();

                if j <= i {
                    end_found = true;
                    break
                }
            }
            if end_found {
                break   
            }
            compacted.push(end_file);
            end_file_b -= 1;
        }
        i+=1;
        curr_file += 1;
    }

    // println!("{:?}", compacted);

    let p1 = compacted.into_iter().enumerate().fold(0u64, |acc, (i, f_n)| {
        acc + (i * f_n) as u64
    });


    println!("p1 = {}", p1);
    
}
