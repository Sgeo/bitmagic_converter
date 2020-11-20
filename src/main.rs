use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

fn read_bm_string<T: Read>(file: &mut T) -> String {
    let size = file.read_u32::<BigEndian>().expect("Unable to read string size!");
    let mut bytes: Vec<u8> = vec![0; size as usize];
    file.read_exact(bytes.as_mut_slice()).expect("Unable to read a string!");
    String::from_utf8_lossy(&bytes).into_owned()
}


fn main() {
    let filename = std::env::args().nth(1).expect("Please provide a filename!");
    let metadata_filename_opt = std::env::args().nth(2);
    let mut input_file = File::open(&filename).expect("Unable to open file!");
    input_file.seek(SeekFrom::Start(4)).expect("Unable to seek!");
    let header_size = input_file.read_u32::<BigEndian>().expect("Unable to read BitMagic header size!");
    input_file.seek(SeekFrom::Start(0x18)).expect("Unable to seek!");
    let ext = read_bm_string(&mut input_file);
    let name = read_bm_string(&mut input_file);
    let creator = read_bm_string(&mut input_file);
    let edition: u32 = input_file.read_u32::<BigEndian>().expect("Unable to read edition!");

    input_file.seek(SeekFrom::Start(header_size.into())).expect("Unable to seek!");
    let mut output_file = File::create(format!("{}.{}", &filename, ext)).expect("Unable to create output file!");
    std::io::copy(&mut input_file, &mut output_file).expect("Unable to copy data!");

    if let Some(metadata_filename) = metadata_filename_opt {
        let mut metadata_file = std::fs::OpenOptions::new().create(true).append(true).open(metadata_filename).expect("Unable to open metadata file!");
        write!(metadata_file, "File: {}\nName: {}\nCreator: {}\nEdition: {}\n\n", &filename, &name, &creator, &edition);
    } else {
        println!("File: {}\nName: {}\nCreator: {}\nEdition: {}\n\n", &filename, &name, &creator, &edition);
    }
}
