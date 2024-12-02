advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
   let reports = parse_input(input);
   Some(reports
   .iter()
   .filter(|report| {
        find_unsafe_level(report, None).is_none()
   })
   .count() as u32)
}

// Returns None if it's safe, or the index of the 2 *center* elements that made the report unsafe.
fn find_unsafe_level(report: &[u32], index_to_ignore: Option<usize>) -> Option<(usize, usize)> {
    let mut i = 0;
    let mut j = 1;
    let mut positives = 0;
    let mut negatives = 0;

    while i < report.len() - 1 && j < report.len() {
        if Some(i) == index_to_ignore {
            i += 1;
            continue;
        }

        if Some(j) == index_to_ignore {
            j += 1;
            continue;
        }

        if i == j {
            j += 1;
            continue;
        }

        let diff = report[i] as i32 - report[j] as i32;
        if diff.abs() < 1 || diff.abs() > 3 {
            return Some((i, j));
        } 
        
        if diff > 0 {
            positives += 1;
        } else {
            negatives += 1;
        }

        if positives > 0 && negatives > 0 {
            return Some((i, j));
        }

        i += 1;
        j += 1;
    }

    if positives > 0 && negatives > 0 {
        return Some((i, j));
    } else {
        return None;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    Some(reports
    .iter()
    .filter(|report| {
        // If we can find a single unsafe level, we can ignore it and check if the report is safe.
        if let Some(indices) = find_unsafe_level(&report, None) {
            let start = indices.0.saturating_sub(1);
            for index_to_ignore in start..=start + 2 {
                match find_unsafe_level(&report, Some(index_to_ignore)) {
                    Some(_) => continue,
                    None => return true
                };
            }
            let start = indices.1.saturating_sub(1);
            for index_to_ignore in start..=start + 2 {
                match find_unsafe_level(&report, Some(index_to_ignore)) {
                    Some(_) => continue,
                    None => return true
                };
            }

            // Full brute-force. Probably better, given the size of the input. But it's more fun to do it the other way.
            // Benchmarks shows exactly no difference in performance.
            // for index_to_ignore in 0 .. report.len() {
            //     match find_unsafe_level(&report, Some(index_to_ignore)) {
            //         Some(_) => continue,
            //         None => return true
            //     };
            // }

            return false;
        } 

        return true;
    })  
    .count() as u32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
