use std::cmp::{Ordering, max};

const INPUT: &'static str = include_str!("../input");
const _TEST: &'static str = include_str!("../test");

fn main() {
    let (ranges, ids) = INPUT.split_once("\n\n").unwrap();
    let mut ranges: Vec<_> = ranges
        .lines()
        .map(|l| {
            let (s, e): (usize, usize) = l
                .split_once("-")
                .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
                .unwrap();
            s..=e
        })
        .collect();
    ranges.sort_by(|a, b| b.start().cmp(a.start()));
    let mut combined_ranges = Vec::with_capacity(ranges.len());
    combined_ranges.push(ranges.pop().unwrap());
    for range in ranges.into_iter().rev() {
        if let Some(last) = combined_ranges.last_mut()
            && last.end() >= range.start()
        {
            // Wir fassen zusammen
            *last = *last.start()..=max(*last.end(), *range.end());
        } else {
            combined_ranges.push(range);
        }
    }

    let mut fresh = 0;
    for id in ids.lines() {
        let id = id.parse().unwrap();
        if combined_ranges.iter().any(|r| r.contains(&id)) {
            fresh += 1;
        }
    }
    println!("{fresh}");

    println!(
        "{}",
        combined_ranges
            .into_iter()
            .fold(0, |acc, r| acc + r.count())
    )
}
