use rpds::map::hash_trie_map::HashTrieMap;
use rpds::stack::Stack;

type CharCounts = HashTrieMap<char, usize>;
type Dictionary<'word> = HashTrieMap<&'word str, CharCounts>;

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

/// Return the set of phrases that can be formed by permuting the letters in input to form words
/// from dictionary.
///
/// ```
/// # use anagram::anagrams;
/// let dictionary = vec!("pet", "er");
/// let mut phrases = anagrams("peter", dictionary.into_iter());
/// phrases.sort(); // hashing order can be nondeterministic
/// assert_eq!(phrases, vec!["er pet", "pet er"]);
/// ```
pub fn anagrams<'word>(
    input: &str,
    dictionary: impl Iterator<Item=&'word str>,
) -> Vec<String> {
    let mut anagrams = Vec::new();
    let input_char_counts = char_counts(input);
    let dictionary_char_counts = dictionary
        .filter(|candidate| input.to_lowercase() != candidate.to_lowercase())
        .map(|candidate| {
            let counts = char_counts(&candidate);
            (candidate, counts)
        })
        .collect();
    anagrams_recurse(input_char_counts, &dictionary_char_counts, &Stack::new(), &mut anagrams);
    anagrams
}

fn anagrams_recurse(
    remaining_chars: CharCounts,
    dictionary: &Dictionary,
    working_phrase: &Stack<&str>,
    anagrams: &mut Vec<String>,
) {
    // The dictionary to be used as we recurse, with words we know aren't worth checking removed
    let mut dictionary_out = dictionary.clone();
    for (word, char_counts) in dictionary.iter() {
        if let Some(working_chars) = deduct(&remaining_chars, char_counts) {
            let working_phrase = working_phrase.push(word);
            if working_chars.is_empty() {
                let mut words = working_phrase.into_iter();
                let mut anagram = words.next().expect("we just pushed a word").to_string();
                for word in words {
                    anagram.push_str(" ");
                    anagram.push_str(word);
                };
                anagrams.push(anagram);
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
