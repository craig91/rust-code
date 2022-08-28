#![allow(non_snake_case)]

use std::io::Write;

fn main() {
   let mut fileRef = std::fs::File::create("sample.txt").expect("create failed");
   
   fileRef.write_all("Hello World\n".as_bytes()).expect("write failed");
   fileRef.write_all("Hello India\n".as_bytes()).expect("write failed");
   
   println!("Text written into file successfully");
}

