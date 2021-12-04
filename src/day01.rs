use crate::parser::*;

pub fn main() -> () {
    let input_a = parse_lines_to_ints("./input/day01/actual.txt");
    let input_b = parse_lines_to_ints("./input/day01/actual.txt");

    println!("01a: {}", count_increases(&input_a, 1));
    println!("01b: {}", count_increases(&input_b, 3));
}

fn count_increases(list: &Vec<i32>, offset: usize) -> i32 {
    list.iter()
        .zip(list.iter().skip(offset))
        .filter(|(first, second)| first < second)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use crate::day01::*;

    #[test]
    fn testcase_a() {
        assert_eq!(
            count_increases(&parse_lines_to_ints("./input/day01/test.txt"), 1),
            7
        );
    }

    #[test]
    fn testcase_b() {
        assert_eq!(
            count_increases(&parse_lines_to_ints("./input/day01/test.txt"), 3),
            5
        );
    }

    #[test]
    fn actual_a() {
        assert_eq!(
            count_increases(&parse_lines_to_ints("./input/day01/actual.txt"), 1),
            1696
        );
    }

    #[test]
    fn actual_b() {
        assert_eq!(
            count_increases(&parse_lines_to_ints("./input/day01/actual.txt"), 3),
            1737
        );
    }
}
