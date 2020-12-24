use parse_display::FromStr;
use std::fs;

#[derive(PartialEq, Debug, FromStr)]
#[display("{min}-{max} {character}: {password}")]
struct PasswordEntry {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

fn main() {
    let input = fs::read_to_string("input/02").unwrap();
    let database = input
        .lines()
        .map(|line| line.parse::<PasswordEntry>().unwrap())
        .collect();

    println!("Solution A: {:?}", solve_a(&database));
    println!("Solution B: {:?}", solve_b(&database));
}

fn solve_a(database: &Vec<PasswordEntry>) -> usize {
    database.iter().filter(|entry| entry.is_valid()).count()
}

fn solve_b(database: &Vec<PasswordEntry>) -> usize {
    database
        .iter()
        .filter(|entry| entry.new_rule_is_valid())
        .count()
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let char_count = self
            .password
            .chars()
            .filter(|&c| c == self.character)
            .count();

        return self.min <= char_count && char_count <= self.max;
    }

    fn new_rule_is_valid(&self) -> bool {
        return self
            .password
            .chars()
            .nth(self.min - 1)
            .map_or(false, |c| c == self.character)
            ^ self
                .password
                .chars()
                .nth(self.max - 1)
                .map_or(false, |c| c == self.character);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password_when_valid() {
        let password_entry = PasswordEntry {
            min: 0,
            max: 9,
            character: 'a',
            password: String::from("aaabcde"),
        };

        assert_eq!(password_entry.is_valid(), true);
    }

    #[test]
    fn test_validate_password_when_invalid() {
        let password_entry = PasswordEntry {
            min: 0,
            max: 2,
            character: 'a',
            password: String::from("aaabcde"),
        };

        assert_eq!(password_entry.is_valid(), false);
    }

    #[test]
    fn test_validate_password_new_rule_when_valid() {
        let password_entry = PasswordEntry {
            min: 1,
            max: 2,
            character: 'a',
            password: String::from("aaabcde"),
        };

        assert_eq!(password_entry.new_rule_is_valid(), true);
    }

    #[test]
    fn test_validate_password_new_rule_when_invalid() {
        let password_entry = PasswordEntry {
            min: 1,
            max: 2,
            character: 'b',
            password: String::from("aaabcde"),
        };

        assert_eq!(password_entry.new_rule_is_valid(), false);
    }

    #[test]
    fn test_validate_password_new_rule_when_outside_of_range() {
        let password_entry = PasswordEntry {
            min: 8,
            max: 9,
            character: 'a',
            password: String::from("aaabc"),
        };

        assert_eq!(password_entry.new_rule_is_valid(), false);
    }
}
