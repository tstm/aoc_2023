use day20::part2::run;

pub fn main() {
    let input = include_str!("../../input.txt");
    let result = run(input).unwrap();
    println!("Output: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_example1_input() {
    //     let input = include_str!("../../example1.txt");
    //     let result = run(input).unwrap();
    //     assert_eq!(result, 32000000);
    // }
    //
    // #[test]
    // fn test_example2_input() {
    //     let input = include_str!("../../example2.txt");
    //     let result = run(input).unwrap();
    //     assert_eq!(result, 11687500);
    // }
}
