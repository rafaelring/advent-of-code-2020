use std::fs;

fn main() {
    let input = fs::read_to_string("input/01").unwrap();

    let entries = input.lines().map(|line| line.parse().unwrap()).collect();

    println!("Solution 1: {:?}", solve(&entries));
}

fn solve(input: &Vec<i32>) -> i32 {
    let (x, y) = find_two_numbers_sum(2020, &input).unwrap();
    x * y
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
}
