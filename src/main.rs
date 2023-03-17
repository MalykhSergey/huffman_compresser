mod huffman_compressing;
mod utils;
use std::{env, path::Path, time::Instant};
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Недостаточно аргументов!");
        return;
    }
    const BUFFER_SIZE: usize = 1000 * 1000 * 100;
    for i in 1..args.len() {
        let file_name = Path::new(args.get(i).unwrap()).display().to_string();
        if &file_name[file_name.len() - 3..file_name.len()] == "hfm" {
            huffman_compressing::decompress(file_name, BUFFER_SIZE);
        } else {
            huffman_compressing::compress(file_name, BUFFER_SIZE);
        }
    }
    println!("{:?}",start.elapsed());
}