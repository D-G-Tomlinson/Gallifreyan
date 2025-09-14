use std::f64::consts::PI;
use std::io;
mod conversion;
mod tree;
mod shape;
mod draw_word;
mod draw_number;
mod draw_sentence;

use conversion::get_image;

use std::fs::File;
use std::io::prelude::*;

use shape::{Cart,Polar};

fn test() {
    let tests = vec![(72, -56), (-57, 46), (-16, 23), (44, -26), (-61, 84), (-17, -79), (81, 2), (-5, 15), (86, 96), (-98, 75),
                     (3,-4),(-3,-4),(-3,4),(3,4),
                     (1,0),(0,-1),(-1,0),(0,1)];
    for test in tests {
        let (x,y) = test;
        let mid = Polar::from(Cart::new(x as f64,y as f64));
        let result = Cart::from(mid);
        let result:(i32,i32) = (result.x.round() as i32,result.y.round() as i32);
        let mid = mid.theta*180f64/PI;
        let good = if  test == result{"PASS"} else {"FAILED"};
        println!("{:?}->{:?}°->{:?}: {}",test,mid, result,good);
    }
}

fn main() {
    //return test();
        println!("Enter text: ");
		let mut input=String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let output=get_image(&input);
        let mut file = File::create("output.html").unwrap();
        file.write_all(b"<!doctype html>
<html lang=\"en-US\">
  <head>
    <meta charset=\"utf-8\" />
    <title>Gallifreyan</title>
  </head>
  <body>
	<h1>Welcome to David's circle</h1>
    <div style=\"width: 500px; height: 500px\">
").unwrap();
    file.write_all(output.as_bytes()).unwrap();
    file.write_all(b"
    </div>
  </body>
</html>
").unwrap();
    }
