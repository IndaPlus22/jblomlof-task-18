/*
Putting hash in a seperete file because why not?!
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