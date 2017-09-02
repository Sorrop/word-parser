pub fn first_word(s: &[u8]) -> &[u8] {

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
