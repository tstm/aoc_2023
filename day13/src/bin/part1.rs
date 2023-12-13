use day13::part1::run;

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
        assert_eq!(result, 405);
    }

    #[test]
    fn test_example_input_midpoint_below() {
        let input = include_str!("../../example2.txt");
        let result = run(input).unwrap();
        assert_eq!(result, 100);
    }
}
