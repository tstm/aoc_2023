use day5::part2::part2;

pub fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input).unwrap();
    println!("Output: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = include_str!("../../example_lowseed.txt");
        let result = part2(input).unwrap();
        assert_eq!(result, 46);
    }
}
