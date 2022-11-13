use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file(filepath: &str) -> String {
    let path = Path::new(filepath);
    let path_display = path.display();
    println!("{path_display}");
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't read {path_display}: {why}"),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("Can't read file contents at {path_display}: {why}"),
        Ok(_) => (),
    };

    return file_contents;
}
