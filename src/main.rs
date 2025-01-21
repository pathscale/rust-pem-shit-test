fn main() {
    let file_path = std::env::var("FILE_LOCATION").unwrap();
    let content = std::fs::read_to_string(file_path).unwrap();

    println!("{content}");
}
