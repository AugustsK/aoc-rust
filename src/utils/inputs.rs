use std::{fs, path};

pub fn get_input(from_path: String, year: u16, day: u8, use_test_input: bool) -> String {
    let path_string = format!(
        "{}/{:04}/{:02}/{}.txt",
        from_path,
        year,
        day,
        if use_test_input {
            "test_input"
        } else {
            "input"
        }
    );
    let file_path = path::Path::new(&path_string);

    fs::read_to_string(file_path)
        .expect(&format!("Could not find input in {}", file_path.display()))
}
