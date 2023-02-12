use encoding_rs::{mem::convert_utf8_to_latin1_lossy, WINDOWS_1252};
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use crate::hash_func::lazy_hash;

/*THIS NEEDS TO BE THE FILEPATH FROM WHERE YOU EXECUTE THE PROGRAM */
const TOKEN_FILE: &str = "helper_files/token.txt";

/*MAKE SURE THIS DIRECTORY EXISTS */
const FILE_LOCATION: &str = "helper_files/index_files/";

/* sett to empty if normal i guess
    To try and see if .txt files saves storage
*/
const FILE_EXTENSION: &str = "";

pub fn compile_that_shit() {
    let mut table = read_token_file();

    for vec in &mut table {
        create_file_for_hash(vec);
    }
}

/// Read the token file and retrieve a hash table.
fn read_token_file() -> Vec<Vec<(String, usize)>> {
    /*
    Inspired from my task-hash
    */

    //the table stores a (String, Vec) where the string is the word and the vec contains indecies
    let mut table: Vec<Vec<(String, usize)>> = Vec::new();

    let file = OpenOptions::new().read(true).open(TOKEN_FILE).unwrap();

    // Decoder provided in task
    let mut buf = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file),
    );
    let mut line = String::new();
    let mut prev = String::new();
    let mut sum = 0;
    let mut _current_line = buf.read_line(&mut line).unwrap();
    while _current_line != 0 {
        let key = line.split_once(" ").unwrap().0;
        let hash = lazy_hash(key);
        if hash >= table.len() {
            //the hash is larger then available hashes just increase the sizes.
            table.resize(hash + 1, Vec::new())
            //plus one since we want to be able to index by hash e.g table[hash]
        }

        if prev != key {
            // it wasnt last, meaning its new.

            // We store the key and how many bytes we need to traverse to find the first instance of the key
            table[hash].push((key.to_string(), sum));
        }
        prev = key.to_string();
        line = String::new();
        sum += _current_line;
        _current_line = buf.read_line(&mut line).unwrap();
    }
    table
}

#[allow(unused_must_use)]
fn create_file_for_hash(vec: &mut Vec<(String, usize)>) {
    if vec.len() == 0 {
        return;
    }
    let hash_string = &lazy_hash(&vec[0].0).to_string();
    let file_location = FILE_LOCATION.to_string();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_location + hash_string + FILE_EXTENSION)
        .unwrap();

    let (_t1, _t2, sorted_table) = sort_into_tree(vec);
    let len_of_entry = _t1 + convert_to_u8_but_werid(_t2).len() + 1; //ADDING ONE SINCE WE DO "<key> <byte_passed>" e.g we need to count a blank.

    let amount_of_entries_as_slice = convert_to_u8_but_werid(sorted_table.len());
    let len_of_entry_as_slice = convert_to_u8_but_werid(len_of_entry);
    /*Write a header containg <amount of entries> <bytes per entry> */
    file.write_all(&amount_of_entries_as_slice);
    file.write_all(&[0]); //blank
    file.write_all(&len_of_entry_as_slice);
    file.write_all(&[0]); //blank

    /*Then for each entrie we write <name____>  //we fill with blankspace.*/

    for entry in vec {
        // This should work, but if doesn't i blame references.
        let mut clone = entry.0.clone();
        let write_bytes = unsafe { clone.as_bytes_mut() };
        convert_utf8_to_latin1_lossy(entry.0.as_bytes(), write_bytes);
        file.write_all(write_bytes);

        // write trailing 0 after the key + one 0 + leading 0 for value
        // so we can read value like 10. By knowing it ends on a specific byte.
        // but we wont. since i just realized there wont be any trailing 0:s,
        // Still keeping it tho cuz short code!
        let _temp = convert_to_u8_but_werid(entry.1);
        for _ in 0..(len_of_entry - entry.0.len() - _temp.len()) {
            file.write_all(&[0]);
        }
        file.write_all(&_temp);
    }
}

/// Returns the len of the string with the longest name, the biggest usize(value), and a vec containing the tree sorted as we want to write it.
/// Destroys the passed vector.
fn sort_into_tree(vec: &mut Vec<(String, usize)>) -> (usize, usize, Vec<(String, usize)>) {
    if vec.len() == 1 {
        (vec[0].0.len(), vec[0].1, vec.clone())
    } else {
        let mut _return_vec = Vec::new();

        let mut second_half = vec.split_off(1 + (vec.len() / 2));

        _return_vec.push(vec.pop().unwrap());

        if vec.len() > second_half.len() {
            // we want to make sure there is the same amount of elements on each side of the root. So we add a empty line here
            // It also works as a stop block, kinda not fully.
            second_half.push((String::new(), 0));
        }

        let ret_lower = sort_into_tree(vec);
        _return_vec.extend(ret_lower.2);

        let ret_higher = sort_into_tree(&mut second_half);
        _return_vec.extend(ret_higher.2);
        (
            ret_lower.0.max(ret_higher.0),
            ret_lower.1.max(ret_higher.1),
            _return_vec,
        )
    }
}


//converts a usize to a vec with u8 such that their sum = the usize
fn convert_to_u8_but_werid(value: usize) -> Vec<u8> {
    let mut _vec: Vec<u8> = Vec::new();
    let mut _current_val = 0;
    loop {
        if _current_val + u8::MAX as usize >= value {
            //we can safely just push the rest
            _vec.push((value - _current_val) as u8);
            return _vec
        }  else {
            _vec.push(u8::MAX);
            _current_val += u8::MAX as usize;
        }
    }
}
