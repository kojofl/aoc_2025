use std::ops::{Add, Sub};

const INPUT: &'static str = include_str!("../input");
const TEST: &'static str = include_str!("../test");

#[derive(Debug, Clone, Copy)]
struct Wheel {
    inner: i32,
}

impl Add<i32> for Wheel {
    type Output = (Wheel, i32);

    fn add(self, rhs: i32) -> Self::Output {
        let inner = self.inner + rhs;
        let c = inner.div_euclid(100);
        (
            Wheel {
                inner: (self.inner + rhs) % 100,
            },
            c,
        )
    }
}

impl Sub<i32> for Wheel {
    type Output = (Wheel, i32);

    fn sub(self, rhs: i32) -> Self::Output {
        let mut c = rhs.div_euclid(100);
        let rhs = rhs % 100;
        if rhs > self.inner {
            if self.inner != 0 {
                c += 1
            }
            (
                Wheel {
                    inner: 100 - (rhs - self.inner),
                },
                c,
            )
        } else {
            let new = self.inner - rhs;
            if new == 0 {
                c += 1;
            }
            (
                Wheel {
                    inner: self.inner - rhs,
                },
                c,
            )
        }
    }
}

fn main() {
    let mut wheel = Wheel { inner: 50 };
    let mut count = 0;
    for line in INPUT.lines() {
        let (op, v) = line.split_at(1);
        let c;
        match op {
            "L" => (wheel, c) = wheel - v.parse().unwrap(),
            "R" => (wheel, c) = wheel + v.parse().unwrap(),
            _ => unreachable!(),
        }
        count += c;
    }
    println!("{count}")
}
