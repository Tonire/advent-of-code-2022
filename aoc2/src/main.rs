use std::fs;
//use std::collections::HashMap;
use std::time::Instant;


fn fight(first: &str, second: &str) -> i32
{
    let mut ret_val: i32 = 0;
    if first.eq("A")
    {
        if second.eq("X")
        {
            ret_val = 1 + 3;
        }
        else if second.eq("Y") 
        {
            ret_val = 2 + 6;
        }
        else if second.eq("Z")
        {
            ret_val = 3 + 0;
        }
    }
    else if first.eq("B")
    {
        if second.eq("X")
        {
            ret_val = 1 + 0;
        }
        else if second.eq("Y")
        {
            ret_val = 2 + 3
        }
        else if second.eq("Z")
        {
            ret_val = 3 + 6;
        }
    }
    else if first.eq("C")
    {
        if second.eq("X")
        {
            ret_val = 1 + 6;
        }
        else if second.eq("Y")
        {
            ret_val = 2 + 0;
        }
        else if second.eq("Z")
        {
            ret_val = 3 + 3;
        }
    }
    ret_val
}

fn fight_v2(first: &str, second: &str) -> i32
{
    if second.eq("X")
    {
        if first.eq("A")
        {
            return fight("A", "Z");
        }
        else if first.eq("B")
        {
            return fight("B", "X");
        }
        else if first.eq("C")
        {
            return fight("C", "Y");
        }
    }
    else if second.eq("Y")
    {
        if first.eq("A")
        {
            return fight("A", "X");
        }
        else if first.eq("B")
        {
            return fight("B", "Y");
        }
        else if first.eq("C")
        {
            return fight("C", "Z");
        }
    }
    else if second.eq("Z")
    {
        if first.eq("A")
        {
            return fight("A", "Y");
        }
        else if first.eq("B")
        {
            return fight("B", "Z");
        }
        else if first.eq("C")
        {
            return fight("C", "X");
        }
    }
    return 0;
}

fn main() 
{   
    // Using the cache map is actually slower!
    //let mut cache_map: HashMap<String, i32> = HashMap::new();
    let input_file: String = fs::read_to_string("input.txt").expect("File not found: input.txt");
    
    let now = Instant::now();
    let vector_results: Vec<i32> = input_file.split("\n").map(|x| 
    {
        let line: Vec<&str> = x.split(" ").collect();
        /*if cache_map.contains_key(x)
        {
            cache_map[x]
        }
        else
        {
            let fight_result: i32 = fight(line[0].trim_end(), line[1].trim_end());
            cache_map.insert(x.to_string(), fight_result);
            fight_result
        }*/
        //fight(line[0].trim_end(), line[1].trim_end())
        fight_v2(line[0].trim_end(), line[1].trim_end())
    }).collect();
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    
    let mut sum_scores: i32 = 0;
    for score in vector_results
    {
        sum_scores += score;
    }

    println!("Total score: {}", sum_scores);
}
