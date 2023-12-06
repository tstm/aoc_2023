use day6::part1::part1;

pub fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input).unwrap();
    println!("Output: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = include_str!("../../example.txt");
        let result = part1(input).unwrap();
        assert_eq!(result, 288);
    }
}
