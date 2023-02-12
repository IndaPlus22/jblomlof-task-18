use std::time;

mod hash_func;
mod compile_files;
fn main() {
    println!("STARTING COMPILE");
    let start = time::Instant::now();
    compile_files::compile_that_shit();
    println!("Took: {} seconds", start.elapsed().as_secs());
}

/* 
fn get_index_file(hash: usize) -> Option<File> {

}
*/