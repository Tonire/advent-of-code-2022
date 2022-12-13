use std::fs;

fn main() 
{   
    let input_file: String = fs::read_to_string("input.txt").expect("File not found: input.txt");
    let vector_results: Vec<i32> = input_file.split("\n").map(|x| 
    {
        
    }).collect();
}
