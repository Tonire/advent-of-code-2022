use std::fs;

#[derive(Clone)]
struct Rucksack
{
    first_compartiment: String,
    second_compartiment: String,
    repeated_item: Vec<Option<char>>
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
    
    fn get_complete_rucksack(&self) -> String
    {
        let mut owned_str: String = self.first_compartiment.to_owned();
        owned_str.push_str(&self.second_compartiment);
        return owned_str;
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

fn get_badge(_group: &Vec<Rucksack>) -> char
{

    return '0';
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
    for ruck in &rucksack_vec
    {
        sum_priorities += get_priority(ruck.repeated_item[0].unwrap());
    }
    println!("{}", sum_priorities);

    let mut group_count: i32 = 0;
    let mut badges_vec: Vec<Rucksack> = vec![];

    for ruck in rucksack_vec
    {
        badges_vec.push(ruck.clone());
        group_count += 1;
        if group_count == 3
        {
            let badge: char = get_badge(&badges_vec);
            group_count = 0;
        }
    }

}