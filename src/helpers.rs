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

#[macro_export]
macro_rules! sign_extend_32 {
  ($num: expr, $starting_size: expr) => {
      (($num as i32) << (32 - $starting_size) >> (32 - $starting_size)) as u32
  }
}

pub fn readFileIntoVec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    
    buffer
}

pub fn get8BitColor (colorToConvert: u8) -> u8 { // Extend a 5-bit color value into an 8-bit color value
  let mut newColor = colorToConvert << 3; // Make the top 5 bits of the 8-bit color equal to the 5-bit color.
  newColor |= (colorToConvert >> 2); // Make the bottom 3 bits of the 8-bit color equal to the top 3 bits of the 5-bit color
  newColor
}