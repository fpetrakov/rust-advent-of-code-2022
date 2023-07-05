use rust_advent_of_code_2022::get_input_from_file;

fn main() {
    let input = get_input_from_file("day_3-part_1.txt");
    let result = input
        .lines()
        .filter_map(|line| {
            let line = line.as_bytes();
            let (left, right) = line.split_at(line.len() / 2);

            left.iter()
                .find(|item| right.contains(item))
                .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 1 + 26,
                } as u32)
        })
        .sum::<u32>();

    println!("{result:?}")
}
