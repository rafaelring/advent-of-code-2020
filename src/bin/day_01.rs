use std::fs;

fn main() {
    let input = fs::read_to_string("input/01").unwrap();

    let entries = input.lines().map(|line| line.parse().unwrap()).collect();

    println!("Solution A: {:?}", solve_a(&entries));
    println!("Solution B: {:?}", solve_b(&entries));
}

fn solve_a(input: &Vec<i32>) -> Option<i32> {
    match find_two_numbers_sum(2020, &input) {
        Some((x, y)) => return Some(x * y),
        None => return None,
    }
}

fn solve_b(input: &Vec<i32>) -> Option<i32> {
    match find_three_numbers_sum(2020, &input) {
        Some((x, y, z)) => return Some(x * y * z),
        None => return None,
    }
}

fn find_two_numbers_sum(desired: i32, data: &Vec<i32>) -> Option<(i32, i32)> {
    data.iter()
        .filter_map(|first| {
            let second = desired - *first;
            if second == *first && data.iter().filter(|&x| *x == second).count() == 1 {
                None
            } else if data.contains(&second) {
                Some((*first, second))
            } else {
                None
            }
        })
        .next()
}

fn find_three_numbers_sum(desired: i32, data: &Vec<i32>) -> Option<(i32, i32, i32)> {
    data.iter()
        .filter_map(|first| {
            let rest = desired - *first;
            match find_two_numbers_sum(rest, &data) {
                Some((second, third)) => return Some((*first, second, third)),
                None => return None,
            }
        })
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_exists() {
        assert_eq!(
            find_two_numbers_sum(100, &vec![30, 42, 50, 58, 82]),
            Some((42, 58))
        )
    }

    #[test]
    fn test_two_sum_not_found() {
        assert_eq!(find_two_numbers_sum(100, &vec![30, 42, 50, 57, 82]), None)
    }

    #[test]
    fn test_three_sum_exists() {
        assert_eq!(
            find_three_numbers_sum(100, &vec![20, 22, 30, 50, 58, 82]),
            Some((20, 22, 58))
        )
    }

    #[test]
    fn test_three_sum_not_found() {
        assert_eq!(
            find_three_numbers_sum(100, &vec![18, 22, 30, 50, 58, 82]),
            None
        )
    }
}
