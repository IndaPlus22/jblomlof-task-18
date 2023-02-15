/**
 * Program to fast get the amount of uses of a word and the context of the usage.
 * See: https://github.com/IndaPlus22/AssignmentInstructions-Rust/tree/master/structures-task-18
 * Author: Jonathan Blomlöf <jblomlof@kth.se>
 */

/*
I had issues with words that i think should be 0. So instead:
Imma do it myself
WHO THE FUCK DIDNT ENCODE TOKEN.txt IN EITHER LATIN-1 OR NORMAL UTF-8
WTF
 */

pub fn map_from_token_to_latin(word: &str) -> String {
    // this is just hardcoded

    // AND OFCOURSE 'ä' is same as 'å'
    // AND OFCOURSE SO IS 'ö'
    // i seen from tests that 'ä' and 'å' is in the form of [239,191,189]

    //
    let mut res = String::new();
    for char in word.bytes() {
        match char {
            189 => {
                // delete leading and push
                res.pop();
                res.pop();
                res.push(123 as char); // 7B
                //res.push('ä');
            }
            _ => {
                res.push(char as char);
            }
        }
    }
    res
}

pub fn map_from_io_to_latin(word: &str) -> String {
    // convert åäö to 228
    let mut res = String::new();
    for char in word.chars() {
        match char {
            'å' => res.push(123 as char), // 7B
            'ä' => res.push(123 as char),
            'ö' => res.push(123 as char),
            _ => {
                res.push(char);
            }
        }
    }
    res
}
