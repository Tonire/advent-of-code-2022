use std::fs;

#[derive(Clone)]
struct Rucksack
{
    complete_rucksack: String,
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
            if self.second_compartiment.contains(letter)
            {
                self.repeated_item.push(Some(letter));
                println!("Found repeated item! {} -> prio: {}", letter, get_priority(letter));

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

fn get_badge(_group: Vec<Rucksack>) -> Result<char, String>
{
    let ruck: &Rucksack = &_group[0];
    for first_char in ruck.complete_rucksack.chars()
    {
        let ruck_second: &Rucksack = &_group[1];
        let ruck_third: &Rucksack = &_group[2];
        if ruck_second.complete_rucksack.contains(first_char) && ruck_third.complete_rucksack.contains(first_char)
        {
            return Ok(first_char);
        }
    }
    Err("Badge not found".to_string())
}

fn main() 
{
    let file_contents: String = fs::read_to_string("input.txt").expect("File not found: input.txt");
    let rucksack_vec: Vec<Rucksack> = file_contents.split("\n").map(|line|
    {
        let split_line: (&str, &str) = line.split_at(line.len()/2);
        let mut new_rucksack = Rucksack {
            complete_rucksack: String::from(line.trim_end()),
            first_compartiment: String::from(split_line.0.trim_end()),
            second_compartiment: String::from(split_line.1.trim_end()),
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
    println!("Total priorities: {}", sum_priorities);

    let mut i: usize = 0;
    let mut badges_priorities = 0;
    while i < rucksack_vec.len()
    {
        let rucksack_group: Vec<Rucksack> = 
            vec![rucksack_vec[i].clone(), rucksack_vec[i+1].clone(), rucksack_vec[i+2].clone()];
        let group_badge: char = match get_badge(rucksack_group)
        {
            Ok(found_badge) => found_badge,
            Err(err_message) => panic!("{}",err_message),
        };
        badges_priorities += get_priority(group_badge);
        i += 3;
    }
    println!("Total badges priorities: {}", badges_priorities);

}
