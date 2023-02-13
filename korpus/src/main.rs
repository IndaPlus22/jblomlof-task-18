/*THIS NEEDS TO BE THE FILEPATH FROM WHERE YOU EXECUTE THE PROGRAM */
const TOKEN_FILE: &str = "helper_files/token.txt";

/* THE PLACE WHERE THIS COMPILER DUMPS ALL THE FILES*/
const FILE_LOCATION: &str = "helper_files/index_files/";

/* Whatever really, EXCEPT NUMBER */
const MASTER: &str = "MASTER";

mod compile_files;
mod hash_func;

use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom},
};

use clap::Parser;
use encoding_rs::mem::convert_utf8_to_latin1_lossy;

#[derive(Parser)]
struct Args {
    command: String,
}

#[derive(PartialEq, Eq)]
enum Flag {
    Equal,
    Left,
    Right,
}
fn main() {
    let args = Args::parse();
    let command = args.command.to_lowercase();

    if command.starts_with('_') {
        compile_files::compile_that_shit();
    } else {
        find_instances(&command);
    }
}

fn find_instances(word: &str) {
    let word_latin_one: Vec<u8> = {
        let mut _word = word.to_string();
        let _bytes = unsafe {_word.as_bytes_mut()};
        convert_utf8_to_latin1_lossy(word.as_bytes(), _bytes);
        _word.bytes().collect()
    };

    let hash_string = hash_func::lazy_hash(word).to_string();
    let mut list_file = OpenOptions::new()
        .read(true)
        .open(FILE_LOCATION.to_string() + &hash_string + "/" + MASTER)
        .unwrap();

    let (amount_of_entries, len_of_entry, header_size) = get_header_info(&mut list_file);

    // OK so we have read header and stored amount of entries and len of entry and size of header.
    // Time to binary search the file.
    if len_of_entry < word.len() {
        // all words in file are less chars than word, no way it exists.
        println!("Word does not exist");
        return;
    }

    let index:usize;

    let mut boundries: [usize; 2] = [0, amount_of_entries - 1];
    let mut search_pointer = amount_of_entries / 2;
    loop {
        let pos = header_size + len_of_entry * search_pointer;
        let current_word = r_exact(&mut list_file, pos, len_of_entry);

        let mut flag = Flag::Equal;
        for i in 0..word_latin_one.len() {
            if word_latin_one[i] < current_word[i] {
                //go left
                flag = Flag::Left;
                break;
            } else if word_latin_one[i] > current_word[i] {
                //go right
                flag = Flag::Right;
                break;
            }
        }
        if flag == Flag::Equal && word_latin_one.len() < len_of_entry {
            if current_word[word_latin_one.len()] != 0 {
                flag = Flag::Left;
            }
        }

        match flag {
            Flag::Equal => {
                index = search_pointer;
                break;
            },
        
            Flag::Left => { 
                if search_pointer == boundries[0] {
                    // nothing on the left
                    println!("Word not found. here1");
                    return;
                }
                boundries[1] = search_pointer - 1;
                let _off_set = (boundries[1] - boundries[0])/ 2;
                search_pointer = boundries[0] + _off_set;
            },

            Flag::Right => {
                if search_pointer == boundries[1] {
                    // nothing more on right
                    println!("Word not found. here");
                    return;
                }
                boundries[0] = search_pointer + 1;
                let _off_set = (boundries[1] - boundries[0])/ 2;
                search_pointer = boundries[0] + _off_set;
            }
        }
    }
    //now we got the index file. Read it and then read token.txt.
    println!("{}, amount: {}", index, amount_of_entries);
}

/// Returns (amount_of_entries, len_of_entry, header_size)
fn get_header_info(file: &mut File) -> (usize, usize, usize) {
    let mut header_size = 2; // 2 spaces in header
    let mut read_byte: [u8; 1] = [0];
    let mut bytes: Vec<u8> = Vec::new();
    loop {
        file.read_exact(&mut read_byte).unwrap();
        if read_byte[0] == 0 {
            break;
        }
        bytes.push(read_byte[0]);
    }
    let amount_of_entries: usize = {
        // simply .iter().sum() doesnt work so doing it manually
        let mut sum = 0;
        for val in &bytes {
            sum += *val as usize;
        }
        sum
    };
    header_size += bytes.len();
    bytes = Vec::new();

    // Reapet for len of entry
    loop {
        file.read_exact(&mut read_byte).unwrap();
        if read_byte[0] == 0 {
            break;
        }
        bytes.push(read_byte[0]);
    }
    let len_of_entry: usize = {
        // simply .iter().sum() doesnt work so doing it manually
        let mut sum = 0;
        for val in &bytes {
            sum += *val as usize;
        }
        sum
    };
    header_size += bytes.len();
    (amount_of_entries, len_of_entry, header_size)
}

fn r_exact(file: &mut File, at: usize, amount: usize) -> Vec<u8> {
    let mut byte: [u8; 1] = [0];
    let mut bytes: Vec<u8> = Vec::with_capacity(amount);
    file.seek(SeekFrom::Start(at as u64)).unwrap();

    for _ in 0..amount {
        file.read_exact(&mut byte).unwrap();
        bytes.push(byte[0]);
    }

    bytes
}
