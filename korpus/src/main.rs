/**
 * Program to fast get the amount of uses of a word and the context of the usage.
 * See: https://github.com/IndaPlus22/AssignmentInstructions-Rust/tree/master/structures-task-18
 * Author: Jonathan Blomlöf <jblomlof@kth.se>
 */

/*Make sure these two const are set to where korpus and token.txt is and that they are from where you run the program.*/
const TOKEN_FILE: &str = "helper_files/token.txt";
const KORPUS_FILE: &str = "helper_files/korpus";

/* THE PLACE WHERE THIS COMPILER DUMP ALL THE FILES*/
const FILE_LOCATION: &str = "helper_files/index_files/";

/* Whatever really*/
const KEYS: &str = "KEYS";
const VALUES: &str = "VALUES";

const AMOUNT_OF_CONTEXT_ON_EACH_SIDE: usize = 30;
const AMOUNT_TO_PRINT_STANDARD: usize = 25;
mod compile_files;
mod hash_func;
mod map_latin_one;

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, Read, Seek, SeekFrom},
    time::{Duration, Instant},
};

use clap::Parser;
use hash_func::lazy_hash;
use map_latin_one::map_from_io_to_latin;

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

    //testing purpose
    /*let _v = vec![10000368, 10017347, 10047993, 10058693];
    yeet_out_korpus_content(&_v,1);
    return;*/

    if command.starts_with('_') {
        compile_files::compile_that_shit();
    } else {
        find_instances(&command);
    }
}

fn find_instances(word: &str) {
    let start = Instant::now();
    let word_latin_one = map_from_io_to_latin(word);
    let hash = lazy_hash(&word_latin_one).to_string() + "/";
    let mut duration_to_answer = Duration::new(0, 0);
    if let Some((len, pointer)) = get_index(&word_latin_one, &hash) {
        //now we got the index. Read it and then read korpus.
        let positions = read_index(len, pointer, &hash);
        if positions.len() == 0 {
            panic!("Word not found, but at werid stage. Advise recompiling.");
        }
        duration_to_answer = yeet_out_korpus_content(&positions, word_latin_one.len());
    } else {
        // word not found in hash.
        println!("Word not found.")
    }

    println!(
        "It took: {} ms",
        (start.elapsed() - duration_to_answer).as_millis()
    )
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
    let amount_of_entries: usize = convert_from_weird_base255(&bytes);
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
    let len_of_entry: usize = convert_from_weird_base255(&bytes);
    header_size += bytes.len();

    (amount_of_entries, len_of_entry, header_size)
}

/// reads exact `amount` of bytes from `file` at byte `at`
fn r_exact(file: &mut File, at: usize, amount: usize) -> Vec<u8> {
    let mut byte: [u8; 1] = [0];
    let mut bytes: Vec<u8> = Vec::with_capacity(amount);
    file.seek(SeekFrom::Start(at as u64)).unwrap();

    for _ in 0..amount {
        if file.read_exact(&mut byte).is_err() {
            bytes.resize(amount, 0);
            return bytes;
        }

        bytes.push(byte[0]);
    }

    bytes
}

/// Returns Some(len, pointer) or None
fn get_index(word: &str, hash_str: &str) -> Option<(usize, usize)> {
    let word_latin_one: Vec<u8> = word.as_bytes().iter().map(|x| *x).collect();

    let mut list_file = OpenOptions::new()
        .read(true)
        .open(FILE_LOCATION.to_string() + hash_str + KEYS)
        .unwrap();

    let (amount_of_entries, len_of_entry, header_size) = get_header_info(&mut list_file);

    // OK so we have read header and stored amount of entries and len of entry and size of header.
    // Time to binary search the file.
    if len_of_entry < word_latin_one.len() {
        // all lines in file are less chars than word, no way it exists.
        println!("Word does not exist");
        return None;
    }

    let mut boundries: [usize; 2] = [0, amount_of_entries - 1];
    let mut search_pointer = amount_of_entries / 2;
    loop {
        let pos = header_size + len_of_entry * search_pointer;
        let current_word = r_exact(&mut list_file, pos, word_latin_one.len() + 1);

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
        if flag == Flag::Equal {
            if current_word[word_latin_one.len()] != 0 {
                flag = Flag::Left;
            }
        }

        match flag {
            Flag::Equal => {
                return get_len_and_pointer(&mut list_file, pos, len_of_entry);
            }

            Flag::Left => {
                if search_pointer == boundries[0] {
                    // no elements left to search on the left
                    return None;
                }
                boundries[1] = search_pointer - 1;
                let _off_set = (boundries[1] - boundries[0]) / 2;
                search_pointer = boundries[0] + _off_set;
            }

            Flag::Right => {
                if search_pointer == boundries[1] {
                    // no elementss to search on right
                    return None;
                }
                boundries[0] = search_pointer + 1;
                let _off_set = (boundries[1] - boundries[0]) / 2;
                search_pointer = boundries[0] + _off_set;
            }
        }
    }
}

//helper for when get_index returns correctly.
fn get_len_and_pointer(file: &mut File, pos: usize, len_of_entry: usize) -> Option<(usize, usize)> {
    let buf = r_exact(file, pos, len_of_entry);

    let mut _iter = buf.split(|x| *x == 0).skip(1).filter(|x| x.len() > 0);

    let len_to_read = _iter.next().unwrap();
    let pointer = _iter.next().unwrap_or(&[1]);
    assert!(_iter.next().is_none());

    Some((
        convert_from_weird_base255(&len_to_read.to_vec()),
        convert_from_weird_base255(&pointer.to_vec()),
    ))
}

/// Returns the byte positions for the words. that is in korpus
fn read_index(len: usize, pointer: usize, hash_str: &str) -> Vec<usize> {
    let _path = FILE_LOCATION.to_string() + hash_str + VALUES;
    let mut value_file = OpenOptions::new().read(true).open(_path).unwrap();

    let read_bytes= r_exact(&mut value_file, pointer, len);
    let _iter = read_bytes.split(|x| *x == 0);
    let mut res = Vec::new();
    for pos in _iter {
        if pos.len() == 0 {
            continue;
        }
        res.push(convert_from_weird_base255(&pos.to_vec()));
    }
    res
}

fn yeet_out_korpus_content(offsets: &Vec<usize>, word_len: usize) -> Duration {
    let mut file = OpenOptions::new().read(true).open(KORPUS_FILE).unwrap();

    println!("There are {} occurences of the word.", offsets.len());
    let mut duration_to_answer = Duration::new(0, 0);

    let said_no = {
        if offsets.len() > AMOUNT_TO_PRINT_STANDARD {
            println!(
                " Want to see all of them? (y/n) (only {} will be shown otherwize)",
                AMOUNT_TO_PRINT_STANDARD
            );
            let mut _in = String::new();
            let now = Instant::now();
            std::io::stdin().lock().read_line(&mut _in).unwrap();
            duration_to_answer = now.elapsed();
            _in.trim().to_lowercase() != "y"
        } else {
            false
        }
    };

    for (index, _offset) in offsets.iter().enumerate() {
        if (said_no) && (index == AMOUNT_TO_PRINT_STANDARD) {
            break;
        }
        let start = {
            if AMOUNT_OF_CONTEXT_ON_EACH_SIDE > *_offset {
                0
            } else {
                *_offset - AMOUNT_OF_CONTEXT_ON_EACH_SIDE
            }
        };
        let buf = r_exact(
            &mut file,
            start,
            word_len + 2 * AMOUNT_OF_CONTEXT_ON_EACH_SIDE,
        );
        for char_asu8 in buf {
            if char_asu8 == 10 {
                print!(" ");
            } else {
                print!("{}", char_asu8 as char);
            }
        }
        print!("\n");
    }
    duration_to_answer
}

fn convert_from_weird_base255(bytes: &Vec<u8>) -> usize {
    let mut sum = 0;

    for (i, val) in bytes.iter().rev().enumerate() {
        sum += (u8::MAX as usize).pow(i as u32) * ((*val) - 1) as usize;
    }

    sum
}

