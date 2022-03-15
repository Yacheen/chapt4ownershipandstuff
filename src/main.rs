fn main() {

    let my_string = String::from("hewwo wowrd");

    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hewwo wowrd";

    let word = first_word(&my_string_literal[0..]);

    let a = [1, 2, 3, 4, 5];
    
    let slice = &a[1..3];
    println!("{:#?}", slice);

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }
}