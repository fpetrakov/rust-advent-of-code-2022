use rust_advent_of_code_2022::get_input_from_file;

fn main() {
    let input = get_input_from_file("day_1-part_2.txt");
    let mut sums = input
        .split("\n\n")
        .map(|e| e.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();
    sums.sort_unstable();
    let top_three_sum = sums.into_iter().rev().take(3).sum::<u32>();
    println!("{}", top_three_sum);
}
