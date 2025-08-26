use std::io;
mod conversion;
mod tree;
mod shape;
mod draw_shape;

use conversion::get_image;

use std::fs::File;
use std::io::prelude::*;

fn main() {
        println!("Enter text: ");
		let mut input=String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let output=get_image(&input);
        println!("Output is: {}", output);
        let mut file = File::create("svgtest/output.html").unwrap();
        file.write_all(b"<!doctype html>
<html lang=\"en-US\">
  <head>
    <meta charset=\"utf-8\" />
    <title>Gallifreyan</title>
  </head>
  <body>
	<h1>Welcome to David's circle</h1>
").unwrap();
    file.write_all(output.as_bytes()).unwrap();
    file.write_all(b"
  </body>
</html>
").unwrap();
    }
