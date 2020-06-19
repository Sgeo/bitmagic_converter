use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};


fn main() {
    let filename = std::env::args().nth(1).expect("Please provide a filename!");
    let mut input_file = File::open(&filename).expect("Unable to open file!");
    input_file.seek(SeekFrom::Start(4)).expect("Unable to seek!");
    let header_size = input_file.read_u32::<BigEndian>().expect("Unable to read BitMagic header size!");
    input_file.seek(SeekFrom::Start(0x1C)).expect("Unable to seek!");
    let mut ext_bytes: [u8; 3] = [0, 0, 0];
    input_file.read_exact(&mut ext_bytes).expect("Unable to read extension!");
    let ext = std::str::from_utf8(&ext_bytes).expect("Unable to convert extension to UTF-8!");
    input_file.seek(SeekFrom::Start(header_size.into())).expect("Unable to seek!");
    let mut output_file = File::create(format!("{}.{}", &filename, ext)).expect("Unable to create output file!");
    std::io::copy(&mut input_file, &mut output_file).expect("Unable to copy data!");

}
