use std::collections::VecDeque;
use std::fs;

pub fn main() {
    println!("06a: {}", part_1("./input/day06/actual.txt"));
    println!("06b: {}", part_2("./input/day06/actual.txt"));
}

fn part_1(path: &str) -> usize {
    count_fish_after_days(path, 80)
}

fn part_2(path: &str) -> usize {
    count_fish_after_days(path, 256)
}

fn count_fish_after_days(path: &str, days: usize) -> usize {
    let mut raw_count = [0usize; 9];
    fs::read_to_string(path)
        .expect("")
        .to_string()
        .split(',')
        .map(|x| x.parse().unwrap())
        .for_each(|age: usize| {
            raw_count[age] += 1;
        });

    let mut count: VecDeque<usize> = VecDeque::with_capacity(9);
    for age in 0..9 {
        count.push_back(raw_count[age]);
    }

    for _ in 0..days {
        let procreating = count.pop_front().unwrap();
        count[6] += procreating;
        count.push_back(procreating);
    }

    count.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day06::*;

    #[test]
    fn testcase_a() {
        assert_eq!(part_1("./input/day06/test.txt"), 5934)
    }

    #[test]
    fn actual_a() {
        assert_eq!(part_1("./input/day06/actual.txt"), 380243);
    }

    #[test]
    fn actual_b() {
        assert_eq!(part_2("./input/day06/actual.txt"), 1708791884591);
    }

    #[test]
    fn testcase_b() {
        assert_eq!(part_2("./input/day06/test.txt"), 26984457539)
    }
}
