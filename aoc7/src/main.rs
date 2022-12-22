use std::{fs, rc::Rc};

struct File
{
    name: String,
    size: usize
}

struct Directory
{
    name: String,
    parent: Rc<Directory>,
    files: Vec<File>,
    children_dirs: Vec<Directory>,
    size: usize
}

impl Directory
{
    /*fn change_dir(&self, new_dir: &str) -> Result<Directory, String>
    {
        Ok({})
    }*/
    
    /*fn list_dir(&self) -> Result<String, String>
    {

    }*/

    fn parse_dir(&mut self, input: String) -> Result<String, String>
    {
        if input.len() <= 0
        {
            return Err("Empty input. Can't parse!".to_string());
        }
        for line in input.split("\n")
        {
            let split_line: Vec<&str> = line.split(" ").collect();
            if split_line[0].eq("dir")
            {
                let found_dir: Directory = Directory { 
                    name: split_line[1].to_string(),
                    parent: Rc::new(self),
                    files: Vec::new(), 
                    children_dirs: Vec::new(), 
                    size: 0
                };
                self.children_dirs.push(found_dir);
            } 
            else
            {
                let found_file: File = File { 
                    name: split_line[1].to_string(),
                    size: split_line[0].parse::<usize>().unwrap(),
                };
                self.files.push(found_file);
            }
        }
        return Ok(":)".to_string());
    }
}


fn main() 
{
    let mut file_system: Directory = Directory { 
        name: "/".to_string(), 
        files: Vec::new(), 
        children_dirs: Vec::new(), 
        size: 0
    };
    let mut current_dir: Directory = file_system;
    let file_content: String = fs::read_to_string("input.txt").expect("File not found!");
    match current_dir.parse_dir("dir a".to_string())
    {
        Ok(_) => println!("a"),
        Err(_) => println!("b"),
    }
}
