use itertools::Itertools;
use num_bigint::BigUint;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    pub fn empty(&self) -> PointSet {
        PointSet {
            dimensions: self.clone(),
            data: BigUint::ZERO,
        }
    }

    pub fn full(&self) -> PointSet {
        PointSet {
            dimensions: self.clone(),
            data: (BigUint::from(1u32) << self.width * self.height) - BigUint::from(1u32),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn to_big_uint(&self, dimensions: &Dimensions) -> BigUint {
        BigUint::from(1u32) << (self.x + self.y * dimensions.width)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            _ => Err(format!(
                "Can only convert <>^v into a Direction, but not {}",
                c
            )),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct PointSet {
    pub dimensions: Dimensions,
    data: BigUint,
}

impl PointSet {
    pub fn from_point(dimensions: &Dimensions, point: &Point) -> Self {
        let Point { x, y } = point;
        if *x >= dimensions.width || *y > dimensions.height {
            panic!("Point out of bounds");
        }

        let data = point.to_big_uint(&dimensions);
        Self {
            dimensions: dimensions.clone(),
            data,
        }
    }

    pub fn from_point_refs<'a>(
        dimensions: &Dimensions,
        points: impl IntoIterator<Item = &'a Point>,
    ) -> Self {
        Self {
            dimensions: dimensions.clone(),
            data: points.into_iter().fold(BigUint::ZERO, |acc, point| {
                let Point { x, y } = point;
                if *x >= dimensions.width || *y > dimensions.height {
                    panic!("Point out of bounds");
                }

                acc | point.to_big_uint(&dimensions)
            }),
        }
    }

    pub fn from_points(dimensions: &Dimensions, points: impl IntoIterator<Item = Point>) -> Self {
        Self {
            dimensions: dimensions.clone(),
            data: points.into_iter().fold(BigUint::ZERO, |acc, point| {
                let Point { x, y } = point;
                if x >= dimensions.width || y > dimensions.height {
                    panic!("Point out of bounds");
                }

                acc | point.to_big_uint(&dimensions)
            }),
        }
    }

    pub fn points(self: &Self) -> Vec<Point> {
        (0..self.dimensions.height)
            .flat_map(
                |y| (0..self.dimensions.width)
                    .map(move |x| Point { x, y })
                    .filter(|p| self.contains(p))
            )
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.data == BigUint::ZERO
    }

    pub fn len(&self) -> usize {
        self.data.count_ones() as usize
    }

    pub fn contains(&self, point: &Point) -> bool {
        &self.data & point.to_big_uint(&self.dimensions) != BigUint::ZERO
    }

    pub fn shift(&self, direction: Direction) -> Self {
        match direction {
            Direction::Left => Self {
                dimensions: self.dimensions.clone(),
                data: self.data.clone() >> 1,
            },
            Direction::Right => {
                Self {
                    dimensions: self.dimensions.clone(),
                    data: self.data.clone() << 1,
                } & self.dimensions.full()
            }
            Direction::Up => Self {
                dimensions: self.dimensions.clone(),
                data: self.data.clone() >> self.dimensions.width,
            },
            Direction::Down => {
                Self {
                    dimensions: self.dimensions.clone(),
                    data: self.data.clone() << self.dimensions.width,
                } & self.dimensions.full()
            }
        }
    }

    pub fn shake(&self) -> Self {
        self |
            &self.shift(Direction::Left) |
            &self.shift(Direction::Right) |
            &self.shift(Direction::Up) |
            &self.shift(Direction::Down)
    }
}

impl Debug for PointSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ ")?;

        let mut first = true;
        for y in 0..self.dimensions.height {
            for x in 0..self.dimensions.width {
                if self.contains(&Point { x, y }) {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }

                    write!(f, "({}, {})", x, y)?;
                }
            }
        }

        write!(f, " }}")?;
        Ok(())
    }
}

impl BitAnd<&PointSet> for &PointSet {
    type Output = PointSet;

    fn bitand(self, rhs: &PointSet) -> Self::Output {
        assert_eq!(self.dimensions, rhs.dimensions);
        PointSet {
            dimensions: self.dimensions.clone(),
            data: &self.data & &rhs.data,
        }
    }
}

impl BitAnd<&PointSet> for PointSet {
    type Output = PointSet;

    fn bitand(self, rhs: &PointSet) -> Self::Output {
        &self & rhs
    }
}

impl BitAnd for PointSet {
    type Output = PointSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}

impl BitOr for &PointSet {
    type Output = PointSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.dimensions, rhs.dimensions);
        PointSet {
            dimensions: self.dimensions.clone(),
            data: &self.data | &rhs.data,
        }
    }
}

impl BitOr<&PointSet> for PointSet {
    type Output = PointSet;

    fn bitor(self, rhs: &PointSet) -> Self::Output {
        &self | rhs
    }
}

impl BitOr for PointSet {
    type Output = PointSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        &self | &rhs
    }
}

impl BitAndAssign<&PointSet> for PointSet {
    fn bitand_assign(&mut self, rhs: &Self) {
        // TODO: just manipulate data?
        *self = &*self & rhs;
    }
}

impl BitOrAssign<&PointSet> for PointSet {
    fn bitor_assign(&mut self, rhs: &Self) {
        // TODO: just manipulate data?
        *self = &*self | rhs;
    }
}

impl Not for &PointSet {
    type Output = PointSet;

    fn not(self) -> Self::Output {
        let PointSet { dimensions, data } = self;
        let data = dimensions.full().data - data;
        PointSet {
            dimensions: dimensions.clone(),
            data,
        }
    }
}

impl Not for PointSet {
    type Output = PointSet;

    fn not(self) -> Self::Output {
        let PointSet { dimensions, data } = self;
        let data = dimensions.full().data - &data;
        PointSet { dimensions, data }
    }
}

pub fn parse_point_sets(lines: &str) -> HashMap<char, PointSet> {
    let rows: Vec<Vec<char>> = lines
        .lines()
        .map(str::chars)
        .map(Iterator::collect)
        .collect();

    let height = rows.len();
    let width = rows
        .iter()
        .next()
        .expect("Could not parse empty rows")
        .len();

    if rows.iter().any(|row| row.len() != width) {
        panic!("Not all rows have the same width");
    }

    let distinct_chars: HashSet<char> = rows.iter().flatten().map(|c| *c).collect();

    distinct_chars
        .into_iter()
        .map(|c| {
            (
                c,
                PointSet::from_points(
                    &Dimensions {
                        width: width,
                        height: height,
                    },
                    rows.iter().enumerate().flat_map(move |(y, row)| {
                        row.iter()
                            .enumerate()
                            .filter(move |(_x, other_c)| **other_c == c)
                            .map(move |(x, _other_c)| Point { x, y })
                    }),
                ),
            )
        })
        .collect()
}

pub fn point_sets_map(sets: &Vec<(char, &PointSet)>) -> String {
    let Some((_, first)) = sets.iter().next() else {
        return String::new();
    };

    let Dimensions { width, height } = first.dimensions;

    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    sets.iter()
                        .filter(|(_, points)| points.contains(&Point { x, y }))
                        .map(|(c, _)| *c)
                        .next()
                        .unwrap_or(' ')
                })
                .collect::<String>()
        })
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_simple_sets() {
        let d = Dimensions {
            width: 3,
            height: 4,
        };

        let empty = d.empty();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);

        let full = d.full();
        assert!(!full.is_empty());
        assert_eq!(full.len(), 12);
    }

    #[test]
    fn test_points() {
        let d = Dimensions {
            width: 2,
            height: 2,
        };

        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 0, y: 1 };
        let p3 = Point { x: 1, y: 0 };
        let p4 = Point { x: 1, y: 1 };

        let s1 = PointSet::from_point(&d, &p1);
        let s2 = PointSet::from_point(&d, &p2);
        let s3 = PointSet::from_point(&d, &p3);
        let s4 = PointSet::from_point(&d, &p4);

        for s in [&s1, &s2, &s3, &s4] {
            assert_eq!(s.len(), 1)
        }

        assert!(s1.contains(&p1));
        assert!(s2.contains(&p2));
        assert!(s3.contains(&p3));
        assert!(s4.contains(&p4));

        assert!(!s1.contains(&p2));
        assert!(!s1.contains(&p3));
        assert!(!s1.contains(&p4));

        assert!(!s2.contains(&p1));
        assert!(!s2.contains(&p3));
        assert!(!s2.contains(&p4));

        assert!(!s3.contains(&p1));
        assert!(!s3.contains(&p2));
        assert!(!s3.contains(&p4));

        assert!(!s4.contains(&p1));
        assert!(!s4.contains(&p2));
        assert!(!s4.contains(&p3));

        let s12 = &s1 | &s2;
        let s13 = &s1 | &s3;

        assert_eq!(s12.len(), 2);
        assert_eq!(s13.len(), 2);

        let s123 = &s12 | &s13;
        assert_eq!(s123.len(), 3);

        let s1_ = &s12 & &s13;
        assert_eq!(s1_, s1);
    }

    #[test]
    fn test_not() {
        let d = Dimensions {
            width: 2,
            height: 2,
        };
        let p1 = PointSet::from_point(&d, &Point { x: 0, y: 0 });
        let p2 = PointSet::from_point(&d, &Point { x: 0, y: 1 });
        let p3 = PointSet::from_point(&d, &Point { x: 1, y: 0 });
        let p4 = PointSet::from_point(&d, &Point { x: 1, y: 1 });

        let not_p1 = !&p1;
        assert_eq!(not_p1.len(), 3);
        assert_eq!(not_p1, &p2 | &p3 | &p4);

        let not_p2 = !&p2;
        assert_eq!(not_p2.len(), 3);
        assert_eq!(not_p2, &p1 | &p3 | &p4);

        let not_p14 = !(&p1 | &p4);
        assert_eq!(not_p14.len(), 2);
        assert_eq!(not_p14, &p2 | &p3);
    }

    #[test]
    fn test_shift() {
        let d = Dimensions {
            width: 3,
            height: 3,
        };
        let ps = PointSet::from_points(&d, [Point { x: 1, y: 1 }, Point { x: 2, y: 2 }]);

        assert_eq!(
            ps.shift(Direction::Up),
            PointSet::from_points(&d, [Point { x: 1, y: 0 }, Point { x: 2, y: 1 }])
        );

        assert_eq!(
            ps.shift(Direction::Down),
            PointSet::from_points(&d, [Point { x: 1, y: 2 }])
        );

        assert_eq!(
            ps.shift(Direction::Left),
            PointSet::from_points(&d, [Point { x: 0, y: 1 }, Point { x: 1, y: 2 }])
        );

        assert_eq!(
            ps.shift(Direction::Right),
            PointSet::from_points(&d, [Point { x: 2, y: 1 }])
        );
    }

    #[test]
    fn test_from_points() {
        let d = Dimensions {
            width: 4,
            height: 4,
        };
        let points = vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 3, y: 1 },
            Point { x: 3, y: 2 },
        ];

        let point_set = PointSet::from_point_refs(&d, &points);

        assert_eq!(point_set.len(), 4);
        assert!(points.iter().all(|p| point_set.contains(p)));
        assert!(!point_set.contains(&Point { x: 0, y: 1 }));
    }

    #[test]
    fn test_parse_point_sets() {
        let map = "AAA.\n.BB.\n.CCC";

        let point_sets = parse_point_sets(map);

        assert_eq!(point_sets.len(), 4);
        assert!(point_sets.values().all(|ps| ps.dimensions
            == Dimensions {
                width: 4,
                height: 3
            }));

        let points_a = point_sets.get(&'A').unwrap();
        let points_b = point_sets.get(&'B').unwrap();
        let points_c = point_sets.get(&'C').unwrap();
        let points_dot = point_sets.get(&'.').unwrap();

        assert_eq!(points_a.len(), 3);
        assert!(points_a.contains(&Point { x: 0, y: 0 }));
        assert!(points_a.contains(&Point { x: 1, y: 0 }));
        assert!(points_a.contains(&Point { x: 2, y: 0 }));

        assert_eq!(points_b.len(), 2);
        assert!(points_b.contains(&Point { x: 1, y: 1 }));
        assert!(points_b.contains(&Point { x: 2, y: 1 }));

        assert_eq!(points_c.len(), 3);
        assert!(points_c.contains(&Point { x: 1, y: 2 }));
        assert!(points_c.contains(&Point { x: 2, y: 2 }));

        assert_eq!(points_dot.len(), 4);

        assert_eq!(
            points_a | points_b | points_c | points_dot,
            points_dot.dimensions.full()
        );

        let s = point_sets_map(&vec![
            ('A', &points_a),
            ('B', &points_b),
            ('C', &points_c),
            ('.', &points_dot)
        ]);

        assert_eq!(&s, map);
    }
}
