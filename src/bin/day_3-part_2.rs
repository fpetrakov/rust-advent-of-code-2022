use rust_advent_of_code_2022::get_input_from_file;

fn main() {
    let input = get_input_from_file("day_3-part_2.txt");
    let elves_with_items: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let result: u32 = elves_with_items
        .chunks(3)
        .filter_map(|elves| {
            elves[0]
                .iter()
                .find(|item| elves[1].contains(item) && elves[2].contains(item))
                .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 1 + 26,
                } as u32)
        })
        .sum();
    println!("{result}")
}
