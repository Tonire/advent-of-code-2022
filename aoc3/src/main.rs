use std::fs;

struct Rucksack
{
    first_compartiment: String,
    second_compartiment: String,
    repeated_item: Vec<Option<char>>,
}
impl Rucksack
{
    fn fill_repeated_item(&mut self)
    {
        for letter in self.first_compartiment.chars()
        {
            for second_letter in self.second_compartiment.chars()
            {
                if letter.eq(&second_letter)
                {
                    self.repeated_item.push(Some(letter));
                    println!("Found repeated item! {} -> prio: {}", letter, get_priority(letter));
                }
            }
        }
    }
}
fn get_priority(item: char) -> i32
{
    let ret_val: i32 = item as i32;
    if item.is_ascii_uppercase()
    {
        return (ret_val - 64) + 26;
    }
    else
    {
        return ret_val - 96;
    }
}
fn main() 
{
    let file_contents: String = fs::read_to_string("input.txt").expect("File not found: input.txt");
    let rucksack_vec: Vec<Rucksack> = file_contents.split("\n").map(|line|
    {
        let split_line: (&str, &str) = line.split_at(line.len()/2);
        let mut new_rucksack = Rucksack {
            first_compartiment: String::from(split_line.0),
            second_compartiment: String::from(split_line.1),
            repeated_item: vec![]
        };
        new_rucksack.fill_repeated_item();
        new_rucksack
    }).collect();

    let mut sum_priorities: i32 = 0;
    for ruck in rucksack_vec
    {
        /*for repeated_opt in ruck.repeated_item
        {
            sum_priorities += get_priority(repeated_opt.unwrap());
        }*/
        sum_priorities += get_priority(ruck.repeated_item[0].unwrap());
    }
    println!("{}", sum_priorities);
    println!("{}", get_priority('g'));
}
