use std::io;
mod conversion;
mod tree;

use conversion::get_image;

fn main() {
        println!("Enter text: ");
		let mut input=String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let output=get_image(&input);
        println!("Output is: {}", output);
    }
