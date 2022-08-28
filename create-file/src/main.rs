fn main() {
    std::fs::File::create("empty.txt").expect("create failed");
    println!("File created successfully");
}
