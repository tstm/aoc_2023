use day11::part2::run;

pub fn main() {
    let input = include_str!("../../input.txt");
    let result = run(input, 1000000).unwrap();
    println!("Output: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = include_str!("../../example.txt");
        let result = run(input, 10).unwrap();
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_example_input_with_larger_multi() {
        let input = include_str!("../../example.txt");
        let result = run(input, 100).unwrap();
        assert_eq!(result, 8410);
    }
}
