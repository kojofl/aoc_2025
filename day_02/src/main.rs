const INPUT: &'static str = include_str!("../input");
const TEST: &'static str = include_str!("../test");

fn main() {
    let v: Vec<u64> = INPUT
        .split(",")
        .flat_map(|range| {
            let (start, stop) = range.split_once("-").expect("valid range");
            (start.parse::<u64>().expect("valid start")..=stop.trim().parse().expect("valid end"))
                .into_iter()
                .filter(|i| {
                    let len = (*i as f64).log10().ceil() as usize;
                    let mid = len / 2;
                    (1..=mid).into_iter().any(|c| {
                        let mut iter = NumIter::new(*i, c).peekable();
                        while let Some(a) = iter.next() {
                            if let Some(peek) = iter.peek() {
                                if a != *peek {
                                    return false;
                                }
                            }
                        }
                        true
                    })
                })
        })
        .collect();
    println!("{}", v.iter().sum::<u64>());
}

struct NumIter {
    inner: u64,
    len: usize,
    chunk_size: usize,
}

impl NumIter {
    pub fn new(num: u64, size: usize) -> Self {
        NumIter {
            inner: num,
            len: (num as f64).log10().ceil() as usize,
            chunk_size: size,
        }
    }
}

impl Iterator for NumIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        if self.len < self.chunk_size as usize {
            self.len = 0;
            return Some(self.inner);
        }

        let op = 10_usize.pow((self.len - self.chunk_size) as u32);
        let res = self.inner / op as u64;
        self.inner = self.inner % op as u64;
        self.len = self.len - self.chunk_size as usize;
        Some(res)
    }
}
