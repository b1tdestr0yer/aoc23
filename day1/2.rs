use std::fs::File;
use std::io::{self, prelude::*, BufReader, Read};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug, Copy, Clone)]
struct Indexes{
    pos: usize,
    num: u32,
}

fn main() -> io::Result<()> {

    let file = File::open("1.txt")?;
    let mut reader = BufReader::new(file);

    let mut sum = 0;

    let searches = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
    ];

    let mut str = String::new();
    reader.read_to_string(&mut str).expect("cannot read string");

    // let mut found = vec![];

    // for (word, num) in searches {
    //     if let Some(pos) = input.find(word){
    //         found.push(Indexes {
    //             pos, num
    //         });
    //     }
    // }

    for line in str.lines(){
        let mut found = searches
        .iter()
        .filter_map(|(word, num)| line.find(word).map(|pos| Indexes {pos, num: *num}))
        .collect::<Vec<_>>();

        

        let mut last_found = searches
        .iter()
        .filter_map(|(word, num)| line.rfind(word).map(|pos| Indexes {pos, num: *num}))
        .collect::<Vec<_>>();

        found.append(&mut last_found);

        found.sort_by_key(|x| x.pos);

        let first = found.first().unwrap();
        let last = found.last().unwrap();

        let number = (first.num * 10) + last.num;
        sum += number;
    }

    println!("{}", sum);
    Ok(())
}
