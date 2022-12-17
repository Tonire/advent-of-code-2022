use std::{fs, thread::current};

fn part1(movements_vector: &Vec<(i32, i32, i32)>)
{
    let mut stacks: Vec<Vec<char>> = vec![
        ['D', 'L', 'J', 'R', 'V', 'G', 'F'].to_vec(), 
        ['T', 'P', 'M', 'B', 'V', 'H', 'J', 'S'].to_vec(),
        ['V', 'H', 'M', 'F', 'D', 'G', 'P', 'C'].to_vec(),
        ['M', 'D', 'P', 'N', 'G', 'Q'].to_vec(),
        ['J', 'L', 'H', 'N', 'F'].to_vec(),
        ['N', 'F', 'V', 'Q', 'D', 'G', 'T', 'Z'].to_vec(),
        ['F', 'D', 'B', 'L'].to_vec(),
        ['M', 'J', 'B', 'S', 'V', 'D', 'N'].to_vec(),
        ['G', 'L', 'D'].to_vec()
    ];
    
    for current_move in movements_vector
    {
        for _ in 0..current_move.0
        {
            if let Some (removed) = stacks[(current_move.1 - 1) as usize].pop()
            {
                stacks[(current_move.2 - 1) as usize].push(removed);
            }
        }
    }

    for stack in stacks
    {
        if stack.len() != 0
        {
            print!("{}", stack.iter().last().unwrap());
        }
    }
    println!("\n");
}

fn part2(movements_vector: &Vec<(i32, i32, i32)>)
{
    let mut stacks: Vec<Vec<char>> = vec![
        ['D', 'L', 'J', 'R', 'V', 'G', 'F'].to_vec(), 
        ['T', 'P', 'M', 'B', 'V', 'H', 'J', 'S'].to_vec(),
        ['V', 'H', 'M', 'F', 'D', 'G', 'P', 'C'].to_vec(),
        ['M', 'D', 'P', 'N', 'G', 'Q'].to_vec(),
        ['J', 'L', 'H', 'N', 'F'].to_vec(),
        ['N', 'F', 'V', 'Q', 'D', 'G', 'T', 'Z'].to_vec(),
        ['F', 'D', 'B', 'L'].to_vec(),
        ['M', 'J', 'B', 'S', 'V', 'D', 'N'].to_vec(),
        ['G', 'L', 'D'].to_vec()
    ];
    
    
    for current_move in movements_vector
    {
        println!("{}, {}, {}", current_move.0, current_move.1, current_move.2);
        let split_location: usize = (current_move.0 ) as usize;

        let from_stack_len = stacks[(current_move.1 - 1) as usize].len();
        let removed = stacks[(current_move.1 - 1) as usize].split_off(from_stack_len - split_location);
        stacks[(current_move.2 - 1) as usize].extend(removed);
    }

    for stack in stacks
    {
        if stack.len() != 0
        {
            print!("{}", stack.iter().last().unwrap());
        }
    }
    println!("\n");
}

fn main()
{
    let file_contents: String = fs::read_to_string("input.txt").expect("File not found!!");
    let movements_vector: Vec<(i32, i32, i32)> = file_contents.split("\n").map(|line| {
        let moves: Vec<i32> = line.split(" ")
        .map(|m| {
            let ret_val = match m.trim_end().parse::<i32>()
            {
                Ok(v) => v,
                Err(_) => -1
            };
            ret_val
        }).filter(|x| { x.is_positive()} ).collect();
        (moves[0], moves[1], moves[2])
    }).collect();

    //     [D]    
    // [N] [C]    
    // [Z] [M] [P]
    //  1   2   3 
    //let mut stacks: Vec<Vec<char>> = vec![['Z', 'N'].to_vec(), ['M', 'C', 'D'].to_vec(), ['P'].to_vec()];

    //     [S] [C]         [Z]            
    // [F] [J] [P]         [T]     [N]    
    // [G] [H] [G] [Q]     [G]     [D]    
    // [V] [V] [D] [G] [F] [D]     [V]    
    // [R] [B] [F] [N] [N] [Q] [L] [S]    
    // [J] [M] [M] [P] [H] [V] [B] [B] [D]
    // [L] [P] [H] [D] [L] [F] [D] [J] [L]
    // [D] [T] [V] [M] [J] [N] [F] [M] [G]
    //  1   2   3   4   5   6   7   8   9 

    part1(&movements_vector);
    part2(&movements_vector);

}
