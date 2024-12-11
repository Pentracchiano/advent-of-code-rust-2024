
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let target : Vec<char> = "XMAS".chars().collect();

    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'X' {
                for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)].iter() {
                    for k in 1..target.len() {
                        let (ny, nx) = (i as i32 + *dy as i32 * k as i32, j as i32 + *dx as i32 * k as i32); // no overflows, and then check to safely cast.
                        if ny >= 0 && ny < matrix.len() as i32 && nx >= 0 && nx < matrix[0].len() as i32 {
                            let (ny, nx) = (ny as usize, nx as usize);
                            if matrix[ny][nx] != target[k] {
                                break;
                            }
                            if k == target.len() - 1 {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    Some(count)
}


fn is_mas(matrix: &Vec<Vec<char>>, i: usize, j: usize, (dy1, dx1): (i32, i32), (dy2, dx2): (i32, i32)) -> bool {
    let new_i1 = i as i32 + dy1;
    let new_j1 = j as i32 + dx1;
    let new_i2 = i as i32 + dy2;
    let new_j2 = j as i32 + dx2;
    if new_i1 < 0 || new_i1 >= matrix.len() as i32 || new_j1 < 0 || new_j1 >= matrix[0].len() as i32 {
        return false;
    }
    if new_i2 < 0 || new_i2 >= matrix.len() as i32 || new_j2 < 0 || new_j2 >= matrix[0].len() as i32 {
        return false;
    }

    let characters = vec![matrix[new_i1 as usize][new_j1 as usize], matrix[new_i2 as usize][new_j2 as usize]];
    characters == vec!['S', 'M'] || characters == vec!['M', 'S']
}


pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut count = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'A' {
                let first = is_mas(&matrix, i, j, (-1, -1), (1, 1));
                let second = is_mas(&matrix, i, j, (-1, 1), (1, -1));
                if first && second {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
