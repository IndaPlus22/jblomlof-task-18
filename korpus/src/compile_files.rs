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
use crate::MASTER;
use crate::TOKEN_FILE;


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
    let mut table: Vec<(usize, Vec<(String, Vec<usize>)>)> = Vec::new();

    let file = OpenOptions::new().read(true).open(TOKEN_FILE).unwrap();

    // Decoder provided in task
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let mut prev = String::new();

    while buf.read_line(&mut line).unwrap() > 0 {
        let _temp = line.split_once(" ").unwrap();
        let key = map_latin_one::map_from_token_to_latin( _temp.0.trim());
       /* DEBUG PRINT
        let _t_vec: Vec<u8> = key.bytes().collect();
        for c in _t_vec {
            print!("{} - {}, ", c, c as char);
        }*/
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
            table[hash].1.push((key.to_string(), Vec::new()));
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
    let hash_string = &lazy_hash(&vec[0].0).to_string();
    let file_location = FILE_LOCATION.to_string() + hash_string;
    // create FILE_LOCATION/hash_string/
    // and file FILE_LOCATION/hash_string/MASTER which contains all keys.
    create_dir(&file_location);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_location + "/" + MASTER)
        .unwrap();

    let len_of_entry = *longest_key_len;
    let mut amount_of_entries_as_slice = convert_to_u8_but_werid(vec.len());
    let len_of_entry_as_slice = convert_to_u8_but_werid(len_of_entry);

    // We are just gonna dump everything into amount_of_entries..
    /*Write a header containg <amount of entries> <bytes per entry> */
    amount_of_entries_as_slice.push(0);
    amount_of_entries_as_slice.extend_from_slice(&len_of_entry_as_slice);
    amount_of_entries_as_slice.push(0);
    /*Then for each entrie we write <name____>  //we fill with blankspace.*/
    let mut file_count = 0;
    for entry in vec {
        // first we write the key in masterfile


        amount_of_entries_as_slice.extend_from_slice(entry.0.as_bytes());

        // write trailing 0 after the key
        for _ in 0..(len_of_entry - entry.0.len()) {
            amount_of_entries_as_slice.push(0);
        }

        //then we write into the value file
        let mut value_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(FILE_LOCATION.to_string() + hash_string + "/" + &file_count.to_string())
            .unwrap();
        let mut write_buf: Vec<u8> = Vec::new();
        for _val in &entry.1 {
            let value = _val.to_be_bytes();
            write_buf.extend_from_slice(&value[(_val.leading_zeros() / 8) as usize..value.len()]);
            //add blank space
            write_buf.push(0);
        }
        value_file.write_all(&write_buf);

        file_count += 1;
    }
    file.write_all(&amount_of_entries_as_slice);
}
/*
Not needed anymore,
It was used to store the file as a tree like structure
But I realized that searching a tree and using binary search is just as fast..., since they are basically the same.
Both start at median value and then accesses the median for half of the current size and so on.

My claim is backed by SO: https://stackoverflow.com/questions/5968937/binary-search-vs-binary-search-tree

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
*/

//converts a usize to a vec with u8 such that they are in base u8::MAX
fn convert_to_u8_but_werid(value: usize) -> Vec<u8> {
    let mut _vec: Vec<u8> = Vec::new();
    let mut _current_val = 0;
    loop {
        if _current_val + u8::MAX as usize >= value {
            //we can safely just push the rest
            _vec.push((value - _current_val) as u8);
            return _vec;
        } else {
            _vec.push(u8::MAX);
            _current_val += u8::MAX as usize;
        }
    }
}
