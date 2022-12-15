use std::fs;

#[derive(Clone)]
struct Section
{
    start: i32,
    end: i32,
}

impl Section
{
    fn contains(self: &Section, other: &Section) -> bool
    {
        if other.start >= self.start && other.end <= self.end 
        {
            return true;
        }
        return false;
    }

    fn overlaps(self: &Section, other: &Section) -> bool
    {
        if self.start <= other.end && self.end >= other.start
        {
            return true;
        }
        return false;
    }
}

fn main() 
{
    let file_contents = fs::read_to_string("input.txt").expect("File not found!");
    let sections_vec: Vec<(Section, Section)> = file_contents.split("\r\n").map(|line|
    {
        let line_split: Vec<Section> = line.split(",").map(|sec_str|{
            let final_section: Vec<&str> = sec_str.split("-").collect();
            let built_section = Section
            {   
                start: final_section[0].parse::<i32>().unwrap(), 
                end: final_section[1].parse::<i32>().unwrap()
            };
            built_section
        }).collect();

        let ret_vec: (Section, Section) = (line_split[0].clone(), line_split[1].clone());
        ret_vec
    }).collect();

    let mut sum_sections_repeated: i32 = 0;
    let mut sum_sections_overlap: i32 = 0;
    for current in sections_vec
    {
        if current.0.contains(&current.1) || current.1.contains(&current.0)
        {
            //println!("Section: {} - {} contains {} - {}", current.0.start, current.0.end, current.1.start, current.1.end);
            sum_sections_repeated += 1;
        }
        if current.0.overlaps(&current.1) || current.1.overlaps(&current.0)
        {
            println!("Section: {} - {} overlaps {} - {}", current.0.start, current.0.end, current.1.start, current.1.end);
            sum_sections_overlap += 1;
        }
    }
    println!("# Repeated sections: {}", sum_sections_repeated);
    println!("# Overlaped sections: {}", sum_sections_overlap);

}
