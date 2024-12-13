use std::collections::BTreeMap;

fn main() {
    // let input = vec![125, 17];
    let input = vec![9759, 0, 256219, 60, 1175776, 113, 6, 92833];
    // let blinks = 25;
    let blinks = 75;

    // TODO replace with cache below see if it's faster
    let mut cache = BTreeMap::<(u8, u64), u64>::new();
    let mut p1_total = 0;
    for x in input {
        p1_total += count_rocks(blinks, x, &mut cache);
    }

    println!("p1 = {}", p1_total);
}

fn count_rocks(mut blinks: u8, rock_num: u64, cache: &mut BTreeMap<(u8, u64), u64>) -> u64 {
    if let Some(result) = cache.get(&(blinks, rock_num)) {
        return *result;
    }
    if blinks == 0 {
        return 1;
    }

    blinks -= 1;
    let num_rocks = match rock_num {
        0 => count_rocks(blinks, 1, cache),

        r => {
            let digits = digits(r);
            if digits % 2 == 0 {
                let (l, r) = split(r, digits);
                let l = count_rocks(blinks, l, cache);
                let r = count_rocks(blinks, r, cache);
                l + r
            } else {
                count_rocks(blinks, rock_num * 2024, cache)
            }
        }
    };

    cache.insert((blinks + 1, rock_num), num_rocks);

    num_rocks
}

fn digits(mut r: u64) -> u64 {
    let mut digits = 0;
    while r > 0 {
        r /= 10;
        digits += 1;
    }

    digits
}

fn split(r: u64, digits: u64) -> (u64, u64) {
    let s = 10u64.pow(digits as u32 / 2);
    return (r / s, r % s);
}

// TODO NOT USED -- try with caching the full path it's probably faster
// this implmentation isn't done
#[derive(Default)]
struct Cache {
    cache: BTreeMap<u64, Vec<u64>>,
}

impl Cache {
    fn insert(&mut self, key: u64, stones: &[u64]) {
        match self.cache.get(&key) {
            Some(curr) => {
                if stones.len() > curr.len() {
                    self.cache.insert(key, stones.to_vec());
                }
            }
            None => {
                self.cache.insert(key, stones.to_vec());
            }
        }
    }

    fn get(&self, key: u64, from_end: usize) -> Option<u64> {
        match self.cache.get(&key) {
            Some(curr) => {
                if curr.len() < from_end {
                    None
                } else {
                    // TODO check off-by-one
                    Some(*curr.get(curr.len() - from_end - 1).clone().unwrap())
                }
            }
            None => None,
        }
    }
}
