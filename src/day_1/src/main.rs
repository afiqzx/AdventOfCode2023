
fn main() {
    // Get data
    let mut data_path: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    data_path.push_str("/data/day_1/input");
    let input: String = String::from_utf8(std::fs::read(data_path.clone()).unwrap()).unwrap();

    let part_one = part_one(input.clone());
    println!("part_one: {}", part_one);

    let part_two = part_two(input.clone());
    println!("part_two: {}", part_two);

}

fn part_one(input: String) -> i32 {

    input.lines()
        .filter(|line| line.len() > 0)
        .map(|line| -> i32 {
            let mut number: [u8; 2] = [0; 2];

            for letter in line.as_bytes().iter() {
                if letter.is_ascii_digit() {
                    number[0] = *letter;
                    break;
                }
            }

            for letter in line.as_bytes().iter().rev() {
                if letter.is_ascii_digit() {
                    number[1] = *letter;
                    break;
                }
            }

            // I am so stupid here. Should've multiply like part two did
            String::from_utf8(number.to_vec()).unwrap().parse().unwrap()
        })
        .sum::<i32>()
}

fn part_two(input: String) -> i32 {

    input.lines()
        .filter(|line| line.len() > 0)
        .map(|line| -> i32 {
            let mut first_number = 0;
            let mut second_number = 0;

            for split in 0..line.len() {
                if let Some(num) = str_to_int(line.split_at(split).1) {
                    first_number = num;
                    break;
                }
            }

            for split in (0..(line.len())).rev() {
                if let Some(num) = str_to_int(line.split_at(split).1) {
                    second_number = num;
                    break;
                }
            }

            first_number * 10 + second_number
        })
        .sum::<i32>()
}

fn str_to_int(value: &str) -> Option<i32> {
    match value {
        _num if value.starts_with("1") || value.starts_with("one") => Some(1), 
        _num if value.starts_with("2") || value.starts_with("two") => Some(2), 
        _num if value.starts_with("3") || value.starts_with("three") => Some(3), 
        _num if value.starts_with("4") || value.starts_with("four") => Some(4), 
        _num if value.starts_with("5") || value.starts_with("five") => Some(5), 
        _num if value.starts_with("6") || value.starts_with("six") => Some(6), 
        _num if value.starts_with("7") || value.starts_with("seven") => Some(7), 
        _num if value.starts_with("8") || value.starts_with("eight") => Some(8), 
        _num if value.starts_with("9") || value.starts_with("nine") => Some(9), 
        _ => None
    }
}


