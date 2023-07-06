use rust_advent_of_code_2022::get_input_from_file;

fn main() {
    let input = get_input_from_file("day_4-part_1.txt");

    let result = input
        .lines()
        .filter(|line| {
            let (elf1, elf2) = line.split_once(",").unwrap();
            let (min1, max1) = elf1.split_once("-").unwrap();
            let (min2, max2) = elf2.split_once("-").unwrap();
            let min1: u32 = min1.parse().unwrap();
            let max1: u32 = max1.parse().unwrap();
            let min2: u32 = min2.parse().unwrap();
            let max2: u32 = max2.parse().unwrap();

            (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
        })
        .count();

    println!("{result}");
}
