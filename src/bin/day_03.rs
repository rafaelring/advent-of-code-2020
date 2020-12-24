use std::fs;

fn main() {
    let input = fs::read_to_string("input/03").unwrap();

    let map = parse_map(&input);

    println!("Solution A: {:?}", solve_a(&map));
}

fn parse_map(input: &String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn solve_a(map: &Vec<Vec<char>>) -> u8 {
    0
}

fn traverse_map(slope: (usize, usize), map: &Vec<Vec<char>>) -> Vec<char> {
    let (slope_x, slope_y) = slope;

    let mut x_pos = 0;
    let mut path = vec![];

    for line in map.iter().step_by(slope_y) {
        x_pos = (x_pos + slope_x) % line.len();
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_map() -> &'static str {
        "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#"
    }

    #[test]
    fn test_parse() {
        let map = parse_map(&String::from(sample_map()));
        assert_eq!(
            map,
            vec![
                ['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
                ['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
                ['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
                ['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'],
                ['.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
                ['.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'],
                ['.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'],
                ['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                ['#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'],
                ['#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'],
                ['.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#']
            ]
        )
    }

    #[test]
    fn test_traverse() {}
}
