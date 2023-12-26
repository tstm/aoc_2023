use day21::part2::run;

pub fn main() {
    let input = include_str!("../../input.txt");
    let result = run((input, 26501365)).unwrap();
    println!("Output: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = include_str!("../../input.txt");
        let result = run((input, 26501365)).unwrap();
        assert_eq!(result, 607334325965751);
    }

    // #[test]
    // fn test_example_input_10() {
    //     let input = include_str!("../../example.txt");
    //     let result = run(input, 10).unwrap();
    //     assert_eq!(result, 50);
    // }
    //
    // #[test]
    // fn test_example_input_50() {
    //     let input = include_str!("../../example.txt");
    //     let result = run(input, 50).unwrap();
    //     assert_eq!(result, 1594);
    // }
    //
    // #[test]
    // fn test_example_input_100() {
    //     let input = include_str!("../../example.txt");
    //     let result = run(input, 100).unwrap();
    //     assert_eq!(result, 6536);
    // }
    //
    // #[test]
    // fn test_example_input_500() {
    //     let input = include_str!("../../example.txt");
    //     let result = run(input, 500).unwrap();
    //     assert_eq!(result, 167004);
    // }
    //
    // #[test]
    // fn test_example_input_1000() {
    //     let input = include_str!("../../example.txt");
    //     let result = run(input, 1000).unwrap();
    //     assert_eq!(result, 668697);
    // }
    //
    // #[test]
    // fn test_example_input_5000() {
    //     let input = include_str!("../../example.txt");
    //     let result = run((input, 5000)).unwrap();
    //     assert_eq!(result, 16733044);
    // }
}
