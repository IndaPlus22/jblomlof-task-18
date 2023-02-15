/**
 * Program to fast get the amount of uses of a word and the context of the usage.
 * See: https://github.com/IndaPlus22/AssignmentInstructions-Rust/tree/master/structures-task-18
 * Author: Jonathan Blomlöf <jblomlof@kth.se>
 */
/* MAKE SURE ALL CONSTANTS ARE SET IN "main.rs" */
use std::{
    fs::{create_dir, create_dir_all, OpenOptions},
    io::{BufRead, BufReader, Write},
    time,
};

use crate::hash_func::lazy_hash;
use crate::map_latin_one;
use crate::FILE_LOCATION;
use crate::KEYS;
use crate::TOKEN_FILE;
use crate::VALUES;

pub fn compile_that_shit() {
    println!("STARTING COMPILE. TIME WILL BE REGISTERED.");
    println!("NOTE: YOUR ANTIMALEWARE MIGHT GO APE SHIT. LOOK IN TASKMANAGER (OR SIMILAR) AND I ADVISE TO TURN IT OFF TO SPEED UP COMPILATION.");
    let start = time::Instant::now();
    create_dir_all(FILE_LOCATION).unwrap();
    let table = read_token_file();

    for vec in &table {
        create_file_for_hash(vec);
    }

    println!(
        "COMPILING COMPLETE\nTook: {} seconds",
        start.elapsed().as_secs()
    );
}

/// Read the token file and retrieve a hash table.
/// Each hash has stores(<amount of chars in longest word>, Vec<(<key>, Vec<values>)>)
fn read_token_file() -> Vec<(usize, Vec<(String, Vec<usize>)>)> {
    /*
    Inspired from my task-hash
    */

    //the table stores a (size_of_longest_key, Vec<(key, vec<values>)>)
    let mut table: Vec<(usize, Vec<(String, Vec<usize>)>)> = Vec::with_capacity(7000);

    let file = OpenOptions::new().read(true).open(TOKEN_FILE).unwrap();

    // Decoder provided in task
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let mut prev = String::new();

    while buf.read_line(&mut line).unwrap() > 0 {
        let _temp = line.split_once(" ").unwrap();
        let key = map_latin_one::map_from_token_to_latin(_temp.0.trim());
        let value = _temp.1.trim();
        let hash = lazy_hash(&key);
        if hash >= table.len() {
            //the hash is larger then available hashes just increase the sizes.
            table.resize(hash + 1, (0, Vec::new()))
            //plus one since we want to be able to index by hash e.g table[hash]
        }

        if prev != key {
            // it wasnt last, meaning its new.
            // We store the key and how many bytes we need to traverse to find the first instance of the key
            table[hash]
                .1
                .push((key.to_string(), Vec::with_capacity(50)));
            if table[hash].0 < key.len() {
                table[hash].0 = key.len();
            }
        }
        // we push the value regardless if its new or not.
        // or well rather we know that the last key we in out hash is def corresponding to our value.
        table[hash]
            .1
            .last_mut()
            .unwrap()
            .1
            .push(value.parse().unwrap());

        prev = key.to_string();
        line = String::new();
    }
    table
}

#[allow(unused_must_use)]
fn create_file_for_hash((longest_key_len, vec): &(usize, Vec<(String, Vec<usize>)>)) {
    if vec.len() == 0 {
        return;
    }

    //stores the len of each entry as bytes
    let mut amount_of_bytes_to_read_as_bytes: Vec<Vec<u8>> = Vec::with_capacity(vec.len());

    let mut max_to_read = 0;

    // stores what byte in VALUES where our values starts, as bytes
    let mut pointers_as_bytes: Vec<Vec<u8>> = Vec::with_capacity(vec.len());
    let mut _passed: usize = 0;

    // stacking all the values on top of eachother with <0> in between.
    // we also store info about it in the above variables.
    let mut write_bytes_for_value: Vec<u8> = Vec::new();

    for entry in vec {
        // the amount of bytes in value entry
        let mut len_of_value_entry = 0;
        for value in &entry.1 {
            let _bytes = convert_to_base_255_as_bytes(*value);
            write_bytes_for_value.extend_from_slice(&_bytes);
            write_bytes_for_value.push(0);

            len_of_value_entry += 1 + _bytes.len();
        }

        max_to_read = max_to_read.max(len_of_value_entry);
        let len_of_value_entry_as_bytes = convert_to_base_255_as_bytes(len_of_value_entry);
        amount_of_bytes_to_read_as_bytes.push(len_of_value_entry_as_bytes);

        let pointer_as_bytes = convert_to_base_255_as_bytes(_passed);
        pointers_as_bytes.push(pointer_as_bytes);

        _passed += len_of_value_entry
    }
    let max_to_read_as_bytes = convert_to_base_255_as_bytes(max_to_read).len();
    let max_amount_of_bytes_in_pointer = pointers_as_bytes.last().unwrap().len();
    let len_of_entry =
        *longest_key_len + 1 + max_to_read_as_bytes + 1 + max_amount_of_bytes_in_pointer;

    /*Write a header containg <amount of entries> <bytes per entry> */
    //start with writing the amount of entries
    let mut key_write_bytes = convert_to_base_255_as_bytes(vec.len());
    key_write_bytes.push(0);
    key_write_bytes.extend_from_slice(&convert_to_base_255_as_bytes(len_of_entry));
    key_write_bytes.push(0);
    /*Then for each entrie we write <name____>  //we fill with blankspace.*/
    for (index, entry) in vec.iter().enumerate() {
        // first we write the key
        key_write_bytes.extend_from_slice(entry.0.as_bytes());

        // write trailing 0 after the key so that length is of max_key
        for _ in 0..(*longest_key_len - entry.0.len()) {
            key_write_bytes.push(0);
        }
        //blank
        key_write_bytes.push(0);

        //more leading 0
        for _ in 0..(max_to_read_as_bytes - amount_of_bytes_to_read_as_bytes[index].len()) {
            key_write_bytes.push(0);
        }

        //write how many to read
        key_write_bytes.extend_from_slice(&amount_of_bytes_to_read_as_bytes[index]);
        key_write_bytes.push(0);

        //write more 0's
        for _ in 0..(max_amount_of_bytes_in_pointer - pointers_as_bytes[index].len()) {
            key_write_bytes.push(0);
        }
        //write the pointer
        key_write_bytes.extend_from_slice(&pointers_as_bytes[index]);
    }

    let hash_string = &lazy_hash(&vec[0].0).to_string();
    let file_location = FILE_LOCATION.to_string() + hash_string + "/";
    // create FILE_LOCATION/hash_string/
    // and file FILE_LOCATION/hash_string/KEYS which contains all keys and the amount of bytes to look in for in VALUES
    // and file FILE_LOCATON/hash_string/VALUES which contains all
    create_dir(&file_location);

    let mut key_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_location.clone() + KEYS)
        .unwrap();

    key_file.write_all(&key_write_bytes);

    let mut value_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_location + VALUES)
        .unwrap();
    value_file.write_all(&write_bytes_for_value);
}

//convert to bytes with the limit
// we need to conserve 0 as a seperator.
fn convert_to_base_255_as_bytes(value: usize) -> Vec<u8> {
    let mut base255: Vec<u8> = Vec::new();
    let mut amount_of_digits = 1;
    while value >= 255_usize.pow(amount_of_digits) {
        amount_of_digits += 1;
    }
    let mut current_left = value;
    for i in (0..amount_of_digits).rev() {
        let _val = current_left / 255_usize.pow(i as u32);
        base255.push(_val as u8 + 1);
        current_left -= _val * 255_usize.pow(i);
    }

    //print!("Converted {} to {:?}\n", value, base255);
    base255
}
