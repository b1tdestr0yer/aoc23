use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn main() -> io::Result<()> {

    let file = File::open("1.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {

        let mut digits = Vec::new();

        for c in line?.chars(){
            if c.is_digit(10) {
                digits.push(c);
            }
        }
        let n1 = digits[0].to_digit(10).unwrap();
        let n2 = digits[digits.len()-1].to_digit(10).unwrap();
        let new = n1 * 10 + n2;
        sum += new;
    }
    println!("{}", sum);
    Ok(())
}
