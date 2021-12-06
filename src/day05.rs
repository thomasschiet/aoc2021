use std::fs;

pub fn main() {
    println!("05a: {}", part_1("./input/day05/actual.txt"));
    println!("05b: {}", part_2("./input/day05/actual.txt"));
}

fn part_1(path: &str) -> i32 {
    let mut map = vec![0u8; 1000 * 1000];

    fs::read_to_string(path)
        .expect(&*format!("Could not find {}", path))
        .lines()
        .map(Line::from)
        .flat_map(|line| line.get_integer_points_straight())
        .for_each(|point| {
            map[(point.0 + point.1 * 1000) as usize] += 1;
        });

    map.into_iter().filter(|val| *val > 1).count() as i32
}
fn part_2(path: &str) -> i32 {
    let mut map = vec![0u8; 1000 * 1000];
    fs::read_to_string(path)
        .expect(&*format!("Could not find {}", path))
        .lines()
        .map(Line::from)
        .flat_map(|line| line.get_integer_points_straight_and_diagonal())
        .for_each(|point| {
            map[(point.0 + point.1 * 1000) as usize] += 1;
        });

    map.into_iter().filter(|val| *val > 1).count() as i32
}

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn get_integer_points_straight(&self) -> impl IntoIterator<Item = (i32, i32)> {
        if self.x1 == self.x2 {
            (std::cmp::min(self.y1, self.y2)..=std::cmp::max(self.y1, self.y2))
                .map(|y| (self.x1, y))
                .collect()
        } else if self.y1 == self.y2 {
            (self.x1..=self.x2).map(|x| (x, self.y1)).collect()
        } else {
            return vec![];
        }
    }

    fn get_integer_points_straight_and_diagonal(&self) -> Vec<(i32, i32)> {
        if self.x1 == self.x2 {
            (std::cmp::min(self.y1, self.y2)..=std::cmp::max(self.y1, self.y2))
                .map(|y| (self.x1, y))
                .collect()
        } else if self.y1 == self.y2 {
            (self.x1..=self.x2).map(|x| (x, self.y1)).collect()
        } else {
            let dy = if self.y2 > self.y1 { 1 } else { -1 };
            let dx = if self.x2 > self.x1 { 1 } else { -1 };

            let slope = dy * dx;

            (self.x1..=self.x2)
                .map(|x_i| (x_i, self.y1 + slope * (x_i - self.x1)))
                .collect()
        }
    }
}

impl From<&str> for Line {
    fn from(input: &str) -> Self {
        let x: Vec<i32> = input
            .to_owned()
            .split(" -> ")
            .flat_map(|m| m.split(','))
            .map(|n| n.parse().unwrap())
            .collect();

        let line = Line {
            x1: x[0],
            y1: x[1],
            x2: x[2],
            y2: x[3],
        };

        if line.x1 > line.x2 {
            Line {
                x1: line.x2,
                y1: line.y2,
                x2: line.x1,
                y2: line.y1,
            }
        } else {
            line
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day05::*;

    #[test]
    fn testcase_a() {
        assert_eq!(part_1("./input/day05/test.txt"), 5)
    }

    #[test]
    fn actual_a() {
        assert_eq!(part_1("./input/day05/actual.txt"), 8350);
    }

    #[test]
    fn actual_b() {
        assert_eq!(part_2("./input/day05/actual.txt"), 19374);
    }

    #[test]
    fn testcase_b() {
        assert_eq!(part_2("./input/day05/test.txt"), 12)
    }
}
