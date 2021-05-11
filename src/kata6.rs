fn fingerprint(word: &str) -> String {
    use std::iter::FromIterator;

    let mut chars = word.chars().collect::<Vec<char>>();
    chars.sort();
    String::from_iter(chars)
}

fn find_anagrams<T, I>(words: T) -> usize
where
    T: Iterator<Item = I>,
    I: AsRef<str>,
{
    use std::collections::HashMap;

    let mut anagrams = HashMap::<String, Vec<String>>::new();

    for word in words {
        let word_ref = word.as_ref();
        anagrams
            .entry(fingerprint(word_ref))
            .or_insert(vec![])
            .push(word_ref.to_string());
    }

    anagrams.values().filter(|x| x.len() > 1).count()
}

#[cfg(test)]
mod test {
    use super::find_anagrams;

    #[test]
    fn test_basic() {
        let words = vec![
            "abcd", //
            "bcda", //
            "dabc", //
            "a",    //
            "'a",   //
            "b",    //
            "c",    //
            "d",    //
            "abc",  //
            "cab",  //
            "ab'c", //
            "cb'a", //
        ];
        assert!(find_anagrams(words.iter()) == 3);
    }

    #[test]
    fn test_wordlist() {
        use std::fs::File;
        use std::io::BufRead;
        use std::io::BufReader;

        let fin = File::open("data/wordlist.txt").unwrap();
        let reader = BufReader::new(&fin);
        let lines = reader.lines().filter_map(|x| x.ok());

        assert!(find_anagrams(lines) == 20680);
    }
}
