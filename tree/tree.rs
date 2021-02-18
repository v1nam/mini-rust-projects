use std::fs;

fn main() {
    walk(&".".to_string(), "".to_string());
}

fn walk(dir: &String, prefix: String) -> () {
    let mut filepaths: Vec<_> = 
        fs::read_dir(&dir)
            .unwrap()
            .map(|x| x.unwrap().path())
            .collect();
    
    filepaths.sort();

    for index in 0..filepaths.len() {
        let name = filepaths[index].file_name().unwrap().to_str().unwrap();

        if &name[0..1] == "." {
            continue;
        }
       
        if index == filepaths.len() - 1 {
            println!("{}└── {}", prefix, name);

            if filepaths[index].is_dir() {
                walk(&format!("{}/{}", dir, name), format!("{}    ", prefix));
            }

        } else {
            println!("{}├── {}", prefix, name);

            if filepaths[index].is_dir() {
                walk(&format!("{}/{}", dir, name), format!("{}│   ", prefix));
            }
        }
    }

}
