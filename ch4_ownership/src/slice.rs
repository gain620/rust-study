fn first_word_string(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub(crate) fn main() {
    let mut s = String::from("hello world");

    let word = first_word_string(&s); // word의 값은 5가 됩니다.

    // s.clear(); // 이제 s는 ""입니다.

    print!("word = {}", word);
    // word는 여전히 5이지만, 더 이상 유효한 값이 아닙니다!

    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작합니다.
    let word = first_word_str(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작합니다.
    let word = first_word_str(&my_string_literal[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word = first_word_str(my_string_literal);
}