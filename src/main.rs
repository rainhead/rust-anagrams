use rpds::{HashTrieMap, Stack};
use std::{fs, io};
use std::path::PathBuf;
use structopt::StructOpt;

type CharCounts = HashTrieMap<char, usize>;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(short, long, default_value = "/usr/share/dict/words")]
    dictionary: PathBuf,
    input: String,
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::from_args();
    // consider (and copy to heap) only words that could make up part of the input word
    let dictionary = fs::read_to_string(&opts.dictionary)?;
    let words = dictionary.lines();
    for phrase in anagrams(&opts.input, words) {
        for (idx, word) in phrase.iter().enumerate() {
            if idx != 0 {
                print!(" ");
            }
            print!("{}", word);
        }
        println!();
    }
    Ok(())
}

fn char_counts(input: &str) -> CharCounts {
    let mut char_counts = CharCounts::new();
    let lower_input = input.to_lowercase();
    for c in lower_input.chars() {
        if let Some(count) = char_counts.get_mut(&c) {
            *count += 1;
        } else {
            char_counts.insert_mut(c, 1);
        }
    }
    char_counts
}

fn deduct(from: &CharCounts, counts: &CharCounts) -> Option<CharCounts> {
    let mut difference = from.clone();
    for (char, subtrahend) in counts.iter() {
        match from.get(&char) {
            Some(minuend) if minuend.eq(subtrahend) => { difference.remove_mut(char); }
            Some(minuend) if minuend.gt(subtrahend) => { difference.insert_mut(*char, minuend - subtrahend); }
            _ => return None
        }
    }
    Some(difference)
}

fn anagrams<'word>(
    input: &str,
    dictionary: impl Iterator<Item=&'word str>
) -> Vec<Stack<&'word str>> {
    let mut anagrams = Vec::new();
    let input_char_counts = char_counts(input);
    let dictionary_char_counts = dictionary
        .filter(|candidate| input.ne(*candidate))
        .map(|candidate| {
            let counts = char_counts(&candidate);
            (candidate, counts)
        })
        .collect();
    anagrams_recurse(input_char_counts, &dictionary_char_counts, &Stack::new(), &mut anagrams);
    anagrams
}

fn anagrams_recurse<'word>(
    remaining_chars: CharCounts,
    dictionary: &HashTrieMap<&'word str, CharCounts>,
    working_phrase: &Stack<&'word str>,
    anagrams: &mut Vec<Stack<&'word str>>
) {
    // The dictionary to be used as we recurse, with words we know aren't worth checking removed
    let mut dictionary_out = dictionary.clone();
    for (word, char_counts) in dictionary.iter() {
        if let Some(working_chars) = deduct(&remaining_chars, char_counts) {
            let working_phrase= working_phrase.push(word);
            if working_chars.is_empty() {
                anagrams.push(working_phrase);
                continue;
            } else {
                anagrams_recurse(working_chars, &dictionary_out, &working_phrase, anagrams);
            }
        } else {
            // don't use this word when recursing in a later iteration
            dictionary_out.remove_mut(word);
        }
    }
}
