use regex::{Match, Regex};

advent_of_code::solution!(3);

#[derive(Debug)]
struct Multiplication {
    a: u32,
    b: u32,
    start_index: usize,
}

fn all_multiplications(input: &str) -> Vec<Multiplication> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    pattern
        .captures_iter(input)
        .map(|cap| {
            let start_index = cap.get(0).unwrap().start();
            Multiplication {
                a: cap[1].parse().unwrap(),
                b: cap[2].parse().unwrap(),
                start_index,
            }
        })
        .collect()
}

fn find_closest(matches: &[Match], index: usize) -> Option<usize> {
    let pos = matches.partition_point(|m| m.start() < index);
    if pos > 0 {
        Some(matches[pos - 1].start())
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(all_multiplications(input)
        .iter()
        .map(|m| m.a * m.b)
        .sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();

    let all_multiplications = all_multiplications(input);
    let all_do: Vec<Match> = do_pattern.find_iter(input).collect();
    let all_dont: Vec<Match> = dont_pattern.find_iter(input).collect();

    let mut accumulation = 0;
    for multiplication in all_multiplications {
        // find the closest, to the left, do and dont with binary search
        let closest_do = find_closest(&all_do, multiplication.start_index);
        let closest_dont = find_closest(&all_dont, multiplication.start_index);

        // if there's a do and it's closer than the dont, return the multiplication
        // if there is neither, return the multiplication
        // if there's a dont and it's closer than the do, continue
        if  closest_dont.is_none() || closest_do.is_some_and(|value| value.abs_diff(multiplication.start_index) < closest_dont.unwrap().abs_diff(multiplication.start_index)) {
            accumulation += multiplication.a * multiplication.b;
            continue;
        }
    }

    Some(accumulation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
