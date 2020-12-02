use std::fmt::Debug;
/// read lines of file
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub type Lines = io::Lines<io::BufReader<File>>;

pub fn read<P: AsRef<Path>>(filename: P) -> Lines {
    let file = File::open(filename).expect("cant open file");
    io::BufReader::new(file).lines()
}

pub fn read_to<P, T>(filename: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    read(filename)
        .map(|x| x.expect("invalid str").parse().expect("cant parse"))
        .collect()
}
