/**
 * Program to fast get the amount of uses of a word and the context of the usage.
 * See: https://github.com/IndaPlus22/AssignmentInstructions-Rust/tree/master/structures-task-18
 * Author: Jonathan Blomlöf <jblomlof@kth.se>
 */

//setting it to a prime.
const MULTIPLIER: usize = 17;

/// Returns hash for the first 3 letters in a lowercase String.
pub fn lazy_hash(word: &str) -> usize {
    let mut hash = 0;
    let mut _chars = word.chars();
    for _ in 0..3 {
        if let Some(char) = _chars.next() {
            hash *= MULTIPLIER;
            hash += char as usize;
        } else {
            break;
        }
    }
    hash
}