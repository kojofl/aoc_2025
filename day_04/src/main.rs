use std::collections::VecDeque;

const INPUT: &'static str = include_str!("../input");
const _TEST: &'static str = include_str!("../test");

fn main() {
    let mut grid = Grid::from(INPUT);
    println!("{}", grid.solve_1());
    println!("{}", grid.solve_2());
}

struct Grid {
    pub inner: VecDeque<Vec<u8>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut inner: VecDeque<Vec<u8>> = value
            .lines()
            .map(|l| {
                b".".into_iter()
                    .chain(l.as_bytes().iter())
                    .copied()
                    .chain(std::iter::once(b'.'))
                    .collect()
            })
            .collect();
        inner.push_front(vec![b'.'; inner[0].len()]);
        inner.push_back(vec![b'.'; inner[0].len()]);
        Self { inner }
    }
}

struct Position {
    x: usize,
    y: usize,
}

enum Transform {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Position {
    const TRANSFORMS: [Transform; 8] = [
        Transform::Top,
        Transform::Bottom,
        Transform::Left,
        Transform::Right,
        Transform::TopLeft,
        Transform::TopRight,
        Transform::BottomLeft,
        Transform::BottomRight,
    ];
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn transform(&self, t: Transform) -> Position {
        match t {
            Transform::Top => Position {
                y: self.y - 1,
                ..*self
            },
            Transform::Bottom => Position {
                y: self.y + 1,
                ..*self
            },
            Transform::Left => Position {
                x: self.x - 1,
                ..*self
            },
            Transform::Right => Position {
                x: self.x + 1,
                ..*self
            },
            Transform::TopLeft => Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Transform::TopRight => Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Transform::BottomLeft => Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Transform::BottomRight => Position {
                x: self.x + 1,
                y: self.y + 1,
            },
        }
    }
}
impl Grid {
    pub fn solve_1(&self) -> u32 {
        let mut metadata: Vec<Vec<Option<u8>>> =
            vec![vec![None; self.inner[0].len()]; self.inner.len()];
        for (y, row) in self.inner.iter().enumerate() {
            for (x, el) in row.iter().enumerate() {
                if *el == b'@' {
                    let mut toilet_count = 0;
                    for transform in Position::TRANSFORMS {
                        let new_pos = Position::new(x, y).transform(transform);
                        if self.inner[new_pos.y][new_pos.x] == b'@' {
                            toilet_count += 1;
                        }
                        if toilet_count >= 4 {
                            break;
                        }
                    }
                    metadata[y][x] = Some(toilet_count);
                }
            }
        }
        metadata.iter().flatten().fold(0, |acc, v| {
            if v.unwrap_or(u8::MAX) < 4 {
                acc + 1
            } else {
                acc
            }
        })
    }
    pub fn solve_2(&mut self) -> u32 {
        let mut metadata: Vec<Vec<Option<u8>>> =
            vec![vec![None; self.inner[0].len()]; self.inner.len()];
        for y in 0..self.inner.len() {
            for x in 0..self.inner[y].len() {
                let el = self.inner[y][x];
                if el == b'@' {
                    let mut toilet_count = 0;
                    for transform in Position::TRANSFORMS {
                        let new_pos = Position::new(x, y).transform(transform);
                        if self.inner[new_pos.y][new_pos.x] == b'@' {
                            toilet_count += 1;
                        }
                    }
                    metadata[y][x] = Some(toilet_count);
                    if toilet_count < 4 {
                        self.inner[y][x] = b'.';
                        let mut queue = VecDeque::new();
                        for transform in Position::TRANSFORMS {
                            let new_pos = Position::new(x, y).transform(transform);
                            if let Some(m) = metadata[new_pos.y][new_pos.x]
                                && m >= 4
                            {
                                queue.push_back((new_pos.y, new_pos.x));
                            }
                        }
                        while let Some((y, x)) = queue.pop_front() {
                            let meta = metadata[y][x].as_mut().unwrap();
                            *meta = meta.saturating_sub(1);
                            if *meta == 3 {
                                self.inner[y][x] = b'.';
                                for transform in Position::TRANSFORMS {
                                    let new_pos = Position::new(x, y).transform(transform);
                                    if let Some(m) = metadata[new_pos.y][new_pos.x]
                                        && m >= 4
                                    {
                                        queue.push_back((new_pos.y, new_pos.x));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        metadata.iter().flatten().fold(0, |acc, v| {
            if v.unwrap_or(u8::MAX) < 4 {
                acc + 1
            } else {
                acc
            }
        })
    }
}
