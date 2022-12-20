use std::{fs, ops::Range};

fn has_repeated_chars(input: &String) -> Option<(usize, usize, char)>
{
    input.chars().enumerate().find_map(|(i, c)| {
        input.chars()
        .enumerate()
        .skip(i + 1)
        .find(|(_, other)| c == *other)
        .map(|(j, _)| (i, j, c))
    })
}

fn get_substring_in_range(input: &String, sub_range: Range<usize>, offset: usize) -> String
{
    let mut sub_chars: String = String::new();
    let max_loop_iterations: usize = input.len();
    let chars_array: Vec<char> = input.chars().collect::<Vec<char>>();
    for j in sub_range
    {
        if (j + offset) < max_loop_iterations
        {
            let current_char = chars_array[j + offset];
            sub_chars.push(current_char);
        }
    }
    return sub_chars;
}

fn extract_message_index(input: &String, input_range: Range<usize>, length: usize)
{
    let mut i: usize = 0;
    let max_loop_iterations: usize = input.len();
    while i <= max_loop_iterations
    {
        let sub_chars: String = get_substring_in_range(input, input_range.to_owned(), i);
        match has_repeated_chars(&sub_chars) {
            None => {
                println!("non-repeated char at i: {}", i + length);
                break;
        },
            Some(_) => {},
        }
        i = i + 1;
    }
}
fn main() 
{
    let file_content: String = fs::read_to_string("input.txt").expect("File not found");
    extract_message_index(&file_content, 0..4, 4);

    extract_message_index(&file_content, 0..14, 14);

}
