use rust_advent_of_code_2022::get_input_from_file;

fn main() {
    let input = get_input_from_file("day_1-part_1.txt");
    let result = input
        .split("\n\n")
        .map(|e| e.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();

    println!("{}", result);
}
