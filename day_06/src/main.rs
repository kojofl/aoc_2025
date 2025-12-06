use std::iter;

const TEST: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let mut tasks: Vec<Vec<u64>> = Vec::new();
    let mut res = 0;
    let mut lines = INPUT.lines().rev();
    let mut op: Vec<_> = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, op)| {
            if op.is_ascii_whitespace() {
                return false;
            }
            true
        })
        .collect();
    op.push((usize::MAX, &b'.'));

    for (i, numbers) in lines.enumerate() {
        let numbers = numbers.as_bytes();
        for (j, window) in op.windows(2).enumerate() {
            if tasks.get(j).is_none() {
                tasks.push(Vec::new());
            }
            let n = if window[1].0 == usize::MAX {
                &numbers[window[0].0..]
            } else {
                &numbers[window[0].0..window[1].0 - 1]
            };
            if i == 0 {
                tasks[j].extend(iter::repeat_n(0, n.len()));
            }
            for (k, pot) in n.iter().enumerate() {
                if tasks[j][k] == 0 {
                    tasks[j][k] += (pot.saturating_sub(b'0') as u64) * 10_u64.pow(0);
                } else {
                    let pow = (tasks[j][k] as f64 + f64::EPSILON).log10().ceil() as u32;
                    tasks[j][k] += (pot.saturating_sub(b'0') as u64) * 10_u64.pow(pow);
                }
            }
        }
    }
    for (n, (_, op)) in tasks.iter().zip(op.iter()) {
        res += n
            .iter()
            .copied()
            .reduce(|acc, v| match op {
                b'*' => acc * v,
                b'+' => acc + v,
                _ => unreachable!(),
            })
            .unwrap_or(0);
    }
    println!("{res}")
}

fn _part_1() {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut res = 0;
    for line in INPUT.lines().rev() {
        for (i, num) in line.split_whitespace().filter(|s| s.len() != 0).enumerate() {
            if numbers.get(i).is_none() {
                numbers.push(Vec::new());
            }
            if num.as_bytes()[0].is_ascii_digit() {
                numbers[i].push(num.parse().unwrap());
            } else {
                res += numbers[i]
                    .iter()
                    .copied()
                    .reduce(|acc, v| match num.as_bytes()[0] {
                        b'*' => acc * v,
                        b'+' => acc + v,
                        _ => unreachable!(),
                    })
                    .unwrap_or(0);
            }
        }
    }
    println!("{res}")
}
