use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn open_file(url: &str) -> String {
    let path = Path::new(url);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why);
    }

    return s;
}

pub fn save_file(url: String, data: &str) -> () {
    let path = Path::new(&url);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    if let Err(why) = file.write_all(data.as_bytes()) {
        panic!("couldn't write to {}: {}", display, why);
    }
}
