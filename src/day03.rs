use std::fs;

pub fn main() {
    println!("03a: {}", part_1("./input/day03/actual.txt"));
    println!("03b: {}", part_2("./input/day03/actual.txt"));
}

fn part_1(path: &str) -> usize {
    let file = fs::read_to_string(path).expect(&*format!("Could not find {}", path));

    let lines = file.lines().count();
    let length = file.lines().take(1).collect::<String>().len();

    let gamma: usize = file
        .lines()
        .map(|el| usize::from_str_radix(el, 2).unwrap())
        .fold(vec![0; length], |acc, val| {
            acc.into_iter()
                .enumerate()
                .map(|(i, count)| count + get_bit_at(val, length - i - 1))
                .collect()
        })
        .iter()
        .map(|ones| *ones > lines - *ones)
        .enumerate()
        .map(|(i, ones)| (ones as usize) << (length as usize - i as usize - 1))
        .sum();

    let mask: usize = (0..length).into_iter().map(|i| 1 << i).sum();
    let epsilon = mask & !gamma;

    gamma * epsilon
}

fn part_2(path: &str) -> usize {
    let file = fs::read_to_string(path).expect(&*format!("Could not find {}", path));

    let length = file.lines().take(1).collect::<String>().len();

    let bits: Vec<usize> = file
        .lines()
        .map(|el| usize::from_str_radix(el, 2).unwrap())
        .collect();

    let o2_generator_rating = get_rating(&bits, length, most_common_bit_in_column);
    let co2_scrubbing_rating = get_rating(&bits, length, least_common_bit_in_column);

    o2_generator_rating * co2_scrubbing_rating
}

fn get_rating<F: Fn(&Vec<usize>, usize) -> usize>(
    bits: &Vec<usize>,
    length: usize,
    discriminator: F,
) -> usize {
    let mut bits = bits.clone();
    let mut column = 0;
    while bits.len() > 1 {
        let bit_pos = length - column - 1;
        let bit = discriminator(&bits, bit_pos);
        bits = bits
            .into_iter()
            .filter(|num| get_bit_at(*num, bit_pos) == bit)
            .collect();
        column += 1;
    }

    bits[0]
}

fn most_common_bit_in_column(bits: &Vec<usize>, idx: usize) -> usize {
    let (zeroes, ones) = compare_zeroes_and_ones(bits, idx);
    if zeroes > ones {
        0
    } else {
        1
    }
}

fn least_common_bit_in_column(bits: &Vec<usize>, idx: usize) -> usize {
    let (zeroes, ones) = compare_zeroes_and_ones(bits, idx);
    if zeroes > ones {
        1
    } else {
        0
    }
}

fn compare_zeroes_and_ones(bits: &Vec<usize>, idx: usize) -> (usize, usize) {
    let ones = bits
        .iter()
        .map(|number| get_bit_at(*number, idx))
        .sum::<usize>();

    let zeroes = bits.len() - ones;

    (zeroes, ones)
}

fn get_bit_at(input: usize, n: usize) -> usize {
    if input & (1 << n) != 0 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::day03::*;

    #[test]
    fn get_bit() {
        assert_eq!(get_bit_at(0b1, 0), 1);
        assert_eq!(get_bit_at(0b1, 1), 0);
        assert_eq!(get_bit_at(0b10, 0), 0);
        assert_eq!(get_bit_at(0b0110, 0), 0);
        assert_eq!(get_bit_at(0b0110, 1), 1);
        assert_eq!(get_bit_at(0b0110, 2), 1);
        assert_eq!(get_bit_at(0b0110, 3), 0);
    }

    #[test]
    fn testcase_a() {
        assert_eq!(part_1("./input/day03/test.txt"), 198)
    }

    #[test]
    fn actual_a() {
        assert_eq!(part_1("./input/day03/actual.txt"), 1082324)
    }

    #[test]
    fn actual_b() {
        assert_eq!(part_2("./input/day03/actual.txt"), 1353024)
    }

    #[test]
    fn testcase_b() {
        assert_eq!(part_2("./input/day03/test.txt"), 230)
    }
}
