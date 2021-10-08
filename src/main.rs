use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;
use std::str::Lines;
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
    let dictionary = read_to_string(&opts.dictionary)?;
    let dictionary_entries = dictionary.lines();
    let input_char_counts = char_counts(&opts.input);

    let anagrams = Anagrams::new(input_char_counts, dictionary_entries);
    traverse(anagrams, &mut Vec::new(), opts.max_words);
    Ok(())
}

fn char_counts(input: &str) -> HashMap<char, usize> {
    let mut char_count = HashMap::with_capacity(input.len());
    for character in input.chars() {
        if character.is_alphabetic() {
            let entry = char_count.entry(character.to_ascii_lowercase()).or_insert(0);
            *entry += 1;
        }
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
            },
            AnagramTree::Leaf(word) => {
                println!("{} {} ", path.join(" "), word);
            },
        }
    }
}

enum AnagramTree<'a> {
    Tree((&'a str, Anagrams<'a>)),
    Leaf(&'a str),
}

struct Anagrams<'a> {
    dictionary_entries: Lines<'a>,
    remaining_chars: HashMap<char, usize>,
}

impl<'a> Iterator for Anagrams<'a> {
    type Item = AnagramTree<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        'words: while let Some(word) = self.dictionary_entries.next() {
            let mut working_chars = self.remaining_chars.clone();
            for c in word.chars() {
                if let Entry::Occupied(entry) = working_chars.entry(c).and_modify(|ct| *ct -= 1) {
                    if *entry.get() == 0 {
                        entry.remove();
                    }
                } else {
                    continue 'words;
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
    fn new(remaining_chars: HashMap<char, usize>, dictionary_entries: Lines<'a>) -> Self {
        Self {
            dictionary_entries,
            remaining_chars,
        }
    }
}
