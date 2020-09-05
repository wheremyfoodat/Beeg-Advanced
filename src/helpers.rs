use std::fs::File;
use std::io::Read;

#[macro_export]
macro_rules! isBitSet {
    ($num:expr, $bit:expr) => {
      (($num & (1 << $bit)) != 0)
    };
}

#[macro_export]
macro_rules! setBit {
    ($num:expr, $bit:expr, $status:expr) => {
      match isBitSet!($num, $bit) == $status {
        false => $num ^ (1 << $bit),
        true  => $num
      }
    };
}

pub fn readFileIntoVec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
