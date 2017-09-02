use std::str;

pub fn parse_words (s: &String) -> Vec<&str> {

    let mut words: Vec<&str> = Vec::new();

    //i will be a cursor that will hold the index to the first character of the next word
    let mut i = 0;
    let size = s.len();
    let raw_bytes = s.as_bytes();

    while i < size {
        let word_byte = first_word(&raw_bytes[i..size]);
        let word = str::from_utf8(word_byte).unwrap();
        words.push(word);
        i = i + word.len() + 1;
    }
    return words;
}

fn first_word(s: &[u8]) -> &[u8] {

    let mut start = 0;

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' || item == b',' || item == b'.' || item == b'?' || item == b'!' {
            if i == 0 {
                start = 1;
            } else {
                return &s[start..i];
            }
        }
    }

    return &s[start..]
}
