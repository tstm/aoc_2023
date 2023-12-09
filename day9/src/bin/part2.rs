use day9::part2::run;

pub fn main() {
    let input = include_str!("../../input.txt");
    let result = run(input).unwrap();
    println!("Output: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = include_str!("../../example.txt");
        let result = run(input).unwrap();
        assert_eq!(result, 2);
    }
}
