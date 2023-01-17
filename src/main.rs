use rpds::HashTrieMap;
use std::collections::btree_map::Entry;
use std::fs::File;
use std::io;
use std::io::{BufRead, Lines};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(short, long, default_value = "/usr/share/dict/words")]
    dictionary: PathBuf,
    input: String,
    #[structopt(short, long)]
    max_words: Option<u8>,
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::from_args();
    let dictionary = File::open(&opts.dictionary)?.lines().collect::<Vec<&str>>();
    let input_char_counts = char_counts(&opts.input);

    let anagrams = Anagrams::new(input_char_counts, &dictionary);
    traverse(anagrams, &mut Vec::new(), opts.max_words);
    Ok(())
}

fn char_counts(input: &str) -> HashTrieMap<char, usize> {
    let mut char_count = HashTrieMap::new();
    let chars = input.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase());
    for c in chars {
        *char_count.entry(c).or_default() += 1;
    }
    char_count
}

fn traverse<'a>(mut anagrams: Anagrams<'a>, path: &mut Vec<&'a str>, remaining_depth: Option<u8>) {
    while let Some(tree) = anagrams.next() {
        match tree {
            AnagramTree::Tree((word, child)) => {
                if remaining_depth != Some(0) {
                    path.push(word);
                    traverse(child, path, remaining_depth.map(|depth| depth - 1));
                    path.pop();
                }
            }
            AnagramTree::Leaf(word) => {
                println!("{} {} ", path.join(" "), word);
            }
        }
    }
}

enum AnagramTree<'a> {
    Tree((&'a str, Anagrams)),
    Leaf(&'a str),
}

struct Anagrams<'a> {
    dictionary_entries: &'a [&'a str],
    remaining_chars: HashTrieMap<char, usize>,
}

impl Iterator for Anagrams {
    type Item = AnagramTree;

    fn next(&mut self) -> Option<Self::Item> {
        'word: while let Some(word) = self.dictionary_entries.next() {
            let mut working_chars = self.remaining_chars.clone();
            for c in word.chars() {
                if let Entry::Occupied(entry) = working_chars.entry(c).and_modify(|ct| *ct -= 1) {
                    if *entry.get() == 0 {
                        entry.remove();
                    }
                } else {
                    continue 'word;
                }
            }
            if working_chars.is_empty() {
                return Some(AnagramTree::Leaf(word));
            }
            return Some(AnagramTree::Tree((word, Anagrams {
                dictionary_entries: self.dictionary_entries.clone(),
                remaining_chars: working_chars.clone(),
            })));
        }
        None
    }
}

impl<'a> Anagrams<'a> {
    fn new(remaining_chars: HashTrieMap<char, usize>, dictionary_entries: &[&str]) -> Self {
        Self {
            dictionary_entries,
            remaining_chars,
        }
    }
}
