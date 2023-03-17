use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_byte_from_buffer(buf_reader: &mut BufReader<File>) -> Option<u8> {
    if buf_reader.buffer().len() == 0 {
        buf_reader.fill_buf().unwrap();
        if buf_reader.buffer().len() == 0 {
            return None;
        }
    }
    let byte = buf_reader.buffer()[0];
    buf_reader.consume(1);
    return Some(byte);
}
pub fn get_byte_or_exit(buf_reader: &mut BufReader<File>) -> u8 {
    let byte = get_byte_from_buffer(buf_reader);
    if byte.is_none() {
        println!(" Файл повреждён! ");
        std::process::exit(-1);
    } else {
        byte.unwrap()
    }
}

pub fn get_frequencies_from_file(buf_reader: &mut BufReader<File>) -> [u64; 256] {
    let mut frequencies: [u64; 256] = [0; 256];
    let mut byte = get_byte_from_buffer(buf_reader);
    while byte.is_some() {
        let letter = byte.unwrap();
        frequencies[letter as usize] += 1;
        byte = get_byte_from_buffer(buf_reader);
    }
    frequencies
}