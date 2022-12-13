use std::fs;
use regex::Regex;

fn main()
{
    let file_contents = fs::read_to_string("calories.txt").expect("File not found");
    let words: Vec<Vec<i32>> = Regex::new(r"\n\s*\n")
    .unwrap()
    .split(&file_contents.to_owned())
    .map(|x|
        {
            Regex::new(r"(\r\n|\r|\n)").unwrap().split(x).map(|i|
                {
                    match i.parse::<i32>()
                    {
                        Ok(a) => a,
                        Err(_err) => 0
                    }
                }).collect()
        }
    ).collect();

    let mut sorted_sum: Vec<i32> = (0..1).collect();
    for first_arr in words
    {
        let mut group_sum: i32 = 0;
        for num in first_arr
        {
            group_sum += num;
        }
        sorted_sum.push(group_sum);
    }
    sorted_sum.sort();
    let inverted_sorted: Vec<i32> = sorted_sum.into_iter().rev().collect();
    println!("{}", inverted_sorted[0]);
    
}
