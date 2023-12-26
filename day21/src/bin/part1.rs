use day21::part1::run;

pub fn main() {
    let input = include_str!("../../input.txt");
    let result = run((input, 64)).unwrap();
    println!("Output: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = include_str!("../../example.txt");
        let result = run((input, 6)).unwrap();
        assert_eq!(result, 16);
    }
}
