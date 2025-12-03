const INPUT: &'static str = include_str!("../input");
const TEST: &'static str = include_str!("../test");

fn main() {
    let part_1 = solve(TEST, 2);
    let part_2 = solve(INPUT, 12);
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

pub fn solve(input: &str, turn_on: usize) -> u64 {
    input
        .lines()
        .map(|bank| {
            let bytes = bank.as_bytes();
            let mut max_idx: Vec<usize> = (0..turn_on).collect();
            let mut start = 1;
            let mut res = 0;
            for (i, idx) in max_idx.iter_mut().enumerate() {
                *idx = start - 1;
                for (j, c) in bytes[start..=bytes.len() - turn_on + i].iter().enumerate() {
                    if bytes[*idx] < *c {
                        *idx = start + j;
                    }
                }
                start = *idx + 2;
                res += (bytes[*idx] - b'0') as u64 * 10_u64.pow((turn_on - 1 - i) as u32);
            }
            res as u64
        })
        .sum()
}
