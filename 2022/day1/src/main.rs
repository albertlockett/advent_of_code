use std::{
    fs,
    str::FromStr,
};

fn main() {
    let list_of_elf_calories= match fs::read_to_string("input.txt") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("biffed it reading the elf calories {:?}", e);
            std::process::exit(1)
        }
    };

    let mut curr_elf_cals = 0;
    let mut top_cal_elf1 = 0;
    let mut top_cal_elf2 = 0;
    let mut top_cal_elf3 = 0;

    for line in list_of_elf_calories.split("\n") {
        match line {
            "" => {
                if curr_elf_cals > top_cal_elf1 {
                    top_cal_elf3 = top_cal_elf2;
                    top_cal_elf2 = top_cal_elf1;
                    top_cal_elf1 = curr_elf_cals;
                } else if curr_elf_cals > top_cal_elf2 {
                    top_cal_elf3 = top_cal_elf2;
                    top_cal_elf2 = curr_elf_cals;
                } else if curr_elf_cals > top_cal_elf3 {
                    top_cal_elf3 = curr_elf_cals;
                }
                curr_elf_cals = 0;
            }
            _ => {
                let cals = u32::from_str(line).unwrap();
                curr_elf_cals += cals;
            }
        };
    };

    println!("part 1 - max elf cals {:?}", top_cal_elf1);
    println!("part 2 - top 3 elf cals {:?}", top_cal_elf1 + top_cal_elf2 + top_cal_elf3);
}
