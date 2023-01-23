use anagram::anagrams;

use std::{fs, io};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(short, long, default_value = "/usr/share/dict/words")]
    dictionary: PathBuf,
    input: String,
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::from_args();
    let dictionary = fs::read_to_string(&opts.dictionary)?;
    let words = dictionary.lines();
    for anagram in anagrams(&opts.input, words) {
        println!("{}", anagram);
    }
    Ok(())
}
