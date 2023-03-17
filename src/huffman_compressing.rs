use crate::utils::{get_byte_from_buffer, get_byte_or_exit, get_frequencies_from_file};
use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader, BufWriter, Seek, Write},
};
#[derive(PartialEq, Eq)]
pub struct HuffmanNode {
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
    pub letter: Option<u8>,
    pub frequency: u64,
}
impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.frequency.cmp(&other.frequency).reverse())
    }
}
impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}
impl HuffmanNode {
    fn build_huffman_tree(frequencies: [u64; 256]) -> HuffmanNode {
        let mut nodes = BinaryHeap::new();
        for i in 0..frequencies.len() {
            if frequencies[i] == 0 {
                continue;
            }
            let node = HuffmanNode {
                left: None,
                right: None,
                letter: Some(i as u8),
                frequency: frequencies[i],
            };
            nodes.push(node);
        }
        while nodes.len() > 1 {
            let left_node = nodes.pop().unwrap();
            let right_node = nodes.pop().unwrap();
            let node = HuffmanNode {
                frequency: right_node.frequency + left_node.frequency,
                letter: None,
                left: Some(Box::from(left_node)),
                right: Some(Box::from(right_node)),
            };
            nodes.push(node);
        }
        nodes.pop().unwrap()
    }
    fn calculate_bites_for_node(
        node: Box<HuffmanNode>,
        bites: Vec<u8>,
        letters_bites: &mut HashMap<u8, Vec<u8>>,
    ) {
        if node.left.is_some() {
            let mut left_value = bites.clone();
            left_value.push(0);
            Self::calculate_bites_for_node(
                Box::from(node.left.unwrap()),
                left_value,
                letters_bites,
            );
        }
        if node.right.is_some() {
            let mut right_value = bites.clone();
            right_value.push(1);
            Self::calculate_bites_for_node(
                Box::from(node.right.unwrap()),
                right_value,
                letters_bites,
            );
        }
        if node.letter.is_some() {
            letters_bites.insert(node.letter.unwrap(), bites.clone());
        }
    }
}

pub fn compress(source_file_name: String, buffer_size: usize) {
    let source_file = match File::open(source_file_name.clone()) {
        Ok(file) => file,
        Err(error) => {
            println!("Не удалось открыть файл: {}", error);
            return;
        }
    };
    let file_size = source_file.metadata().unwrap().len();
    if file_size == 0 {
        println!(" Пустой файл! ");
        return;
    }
    let mut buf_reader = BufReader::with_capacity(buffer_size, source_file);
    let frequencies = get_frequencies_from_file(&mut buf_reader);
    let node = HuffmanNode::build_huffman_tree(frequencies);
    let mut letters_bites: HashMap<u8, Vec<u8>> = HashMap::new();
    HuffmanNode::calculate_bites_for_node(Box::from(node), vec![], &mut letters_bites);
    let compressed_file = match File::create(source_file_name + ".hfm") {
        Ok(file) => file,
        Err(error) => {
            println!("Не удалось создать файл: {}", error);
            return;
        }
    };
    let mut buf_writer = BufWriter::with_capacity(buffer_size, compressed_file);
    buf_reader.rewind().unwrap();
    encode_letters_in_table(
        &file_size,
        letters_bites.len() as u16,
        frequencies,
        &mut buf_writer,
    );
    write_compressed_file(&letters_bites, &mut buf_reader, &mut buf_writer);
}

pub fn decompress(compressed_file_name: String, buffer_size: usize) {
    let compressed_file = match File::open(compressed_file_name.clone()) {
        Ok(file) => file,
        Err(error) => {
            println!("Не удалось открыть файл: {}", error);
            return;
        }
    };
    let decompressed_file =
        match File::create(&compressed_file_name[0..compressed_file_name.len() - 4]) {
            Ok(file) => file,
            Err(error) => {
                println!("Не удалось создать файл: {}", error);
                return;
            }
        };
    let mut buf_reader = BufReader::with_capacity(buffer_size, compressed_file);
    let mut buf_writer = BufWriter::with_capacity(buffer_size, decompressed_file);
    buf_reader.fill_buf().unwrap();
    let mut bytes: [u8; 8] = [0; 8];
    for i in 0..8 {
        bytes[i] = get_byte_or_exit(&mut buf_reader);
    }
    let file_size = u64::from_be_bytes(bytes);
    buf_reader.consume(8);
    let letters_count = get_byte_or_exit(&mut buf_reader);
    let frequences = decode_letters_from_table(letters_count as u16, &mut buf_reader);
    let root = HuffmanNode::build_huffman_tree(frequences);
    if letters_count == 0 {
        write_only_one_letter(file_size, root.letter.unwrap(), &mut buf_writer);
    } else {
        write_decompressed_file(file_size, root, &mut buf_reader, &mut buf_writer)
    };
}

fn encode_letters_in_table(
    file_size: &u64,
    chars_count: u16,
    frequencies: [u64; 256],
    buf_writer: &mut BufWriter<File>,
) {
    buf_writer.write(&file_size.to_be_bytes()).unwrap();
    buf_writer.write(&[(chars_count - 1) as u8]).unwrap();
    for i in 0..256 {
        if frequencies[i] > 0 {
            buf_writer.write(&[i as u8]).unwrap();
            buf_writer.write(&(frequencies[i]).to_be_bytes()).unwrap();
        }
    }
}

fn write_compressed_file(
    letters_bites: &HashMap<u8, Vec<u8>>,
    buf_reader: &mut BufReader<File>,
    buf_writer: &mut BufWriter<File>,
) {
    let mut i = 0;
    let mut byte = 0;
    let mut letter = get_byte_from_buffer(buf_reader);
    while letter.is_some() {
        let bites_of_letter = letters_bites.get(&letter.unwrap()).unwrap();
        for bit in bites_of_letter {
            if i == 8 {
                buf_writer.write(&[byte]).unwrap();
                byte = 0;
                i = 0;
            }
            byte |= bit << i;
            i += 1;
        }
        letter = get_byte_from_buffer(buf_reader);
    }
    buf_writer.write(&[byte]).unwrap();
}

fn decode_letters_from_table(letters_count: u16, buf_reader: &mut BufReader<File>) -> [u64; 256] {
    let mut letters_counter = 0;
    let mut frequencies: [u64; 256] = [0; 256];
    while letters_counter != letters_count + 1 {
        let letter = get_byte_or_exit(buf_reader) as usize;
        let mut bytes: [u8; 8] = [0; 8];
        for i in 0..8 {
            bytes[i] = get_byte_or_exit(buf_reader);
        }
        let frequncy = u64::from_be_bytes(bytes);
        frequencies[letter] = frequncy;
        letters_counter += 1;
    }
    frequencies
}

fn write_only_one_letter(file_size: u64, letter: u8, buf_writer: &mut BufWriter<File>) {
    for _ in 0..file_size {
        buf_writer.write(&[letter]).unwrap();
    }
}

fn write_decompressed_file(
    file_size: u64,
    root: HuffmanNode,
    buf_reader: &mut BufReader<File>,
    buf_writer: &mut BufWriter<File>,
) {
    let mut node = &root;
    let mut letter_counter = 0;
    let mut byte = get_byte_from_buffer(buf_reader);
    while byte.is_some() {
        for i in 0..8 {
            let bit = (byte.unwrap() & (1 << i)) >> i;
            if bit == 1 {
                node = match &node.right {
                    Some(node) => node,
                    None => {
                        println!("Файл повреждён!");
                        return;
                    }
                };
            } else {
                node = match &node.left {
                    Some(node) => node,
                    None => {
                        println!("Файл повреждён!");
                        return;
                    }
                };
            }
            if node.letter.is_some() {
                buf_writer.write(&[node.letter.unwrap()]).unwrap();
                letter_counter += 1;
                if letter_counter == file_size {
                    return;
                }
                node = &root;
            }
        }
        byte = get_byte_from_buffer(buf_reader);
    }
}