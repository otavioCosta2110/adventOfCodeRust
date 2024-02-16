mod puzzle_input;

fn get_numbers(puzzle_formated: Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for line in puzzle_formated{

        let mut numbers_conc: String = "".to_string();

        for letter_num in 0..line.len() {
            let letter = line.chars().nth(letter_num).unwrap();
            if letter.is_numeric(){
               numbers_conc += &letter.to_string();
               println!("1 - {}", numbers_conc);
               break;
            }
        }
        for letter_num in (0..line.len()).rev() {
            let letter = line.chars().nth(letter_num).unwrap();
            if letter.is_numeric(){
               numbers_conc += &letter.to_string();
               println!("2 - {}", numbers_conc);
               break;
            }
        }
    sum += numbers_conc.parse::<i32>().unwrap();
    println!("{} {}", sum, numbers_conc);
    }
    sum
}
fn main() {
    let puzzle_input = puzzle_input::puzzle_input();
    let formated_input: Vec<String> = puzzle_input.to_string().split("\n").map(|line| line.to_string()).collect();
    println!("{:?}", get_numbers(formated_input));

}
