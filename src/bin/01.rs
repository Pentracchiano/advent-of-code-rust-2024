advent_of_code::solution!(1);

// parse input takes the str and returns two vectors of integers. all the items in the first vector are the first number in each line, and all the items in the second vector are the second number in each line.
fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        v1.push(parts.next().unwrap().parse().unwrap());
        v2.push(parts.next().unwrap().parse().unwrap());
    }
    (v1, v2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut v1, mut v2) = parse_input(input);
    v1.sort();
    v2.sort();
    Some(v1
        .iter()
        .zip(v2)
        .map(|(a, b)| a.abs_diff(b))
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (v1, v2) = parse_input(input);
    let mut counter = std::collections::HashMap::new();

    for number in v2 {
        *counter.entry(number).or_insert(0) += 1;
    }

    Some(v1.iter()
        .map(|number| {
            match counter.get(number) {
                Some(count) => number * count,
                None => 0,
            }
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
