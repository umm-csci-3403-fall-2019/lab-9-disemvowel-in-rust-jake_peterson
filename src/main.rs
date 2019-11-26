fn main() {
    println!("Hello, world!");
    let s = String::from("Hello, world");
    let s_disemvowel = disemvowel(&s);

    println!("{:?}",s_disemvowel);
    //println!("s was '{}', and without vowels is '{}'.", s, s_disemvowel);
}

fn disemvowel(s: &str)->String{
    let mut result = Vec:: new();
    let s_disemvoweled = s.clone();

    for c in s_disemvoweled.chars(){
        //let mut s_disemvowel1 = String::from(s);
        if is_not_vowel(&c){
            result.push(c);
        }
    }
    let my_string: String = result.into_iter().collect();
    return my_string;
}
fn is_not_vowel(the_char: &char)-> bool{
    let mut bool_val = true;
    let vec_vowel = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    if vec_vowel.contains(the_char) {
        bool_val = false;
    }
    bool_val

}
// Everything from here down is Rust test code. You shouldn't need to 
// change any of this. 
//
// Use `cargo test` to run all these tests. All the tests will initially 
// fail because there's no definition for the `disemvowel` function. Add
// that up above and work to get the tests to pass. See the lab write-up
// for some tips.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let input = "Hello, world!";
        let expected = "Hll, wrld!";

        assert_eq!(
            expected,
            disemvowel(input)
        );
    }

    #[test]
    fn empty() {
        assert_eq!("", disemvowel(""));
    }

    #[test]
    fn no_vowels() {
        assert_eq!("pqrst", disemvowel("pqrst"));
    }

    #[test]
    fn all_vowels() {
        assert_eq!("", disemvowel("aeiouAEIOUOIEAuoiea"));
    }

    #[test]
    fn morris_minnesota() {
        assert_eq!("Mrrs, Mnnst", disemvowel("Morris, Minnesota"));
    }

    #[test]
    fn handle_punctuation() {
        assert_eq!("n (nxplnd) lphnt!", 
            disemvowel("An (Unexplained) Elephant!"));
    }

    #[test]
    fn handle_unicode() {
        assert_eq!("Sm hrglyphs: 𒐁	𒐌	𒐥	𒑳",
            disemvowel("Some hieroglyphs: 𒐁	𒐌	𒐥	𒑳"));
        assert_eq!("Sm Lnr B: 	𐂀	𐂚	𐃃	𐃺",
            disemvowel("Some Linear B: 	𐂀	𐂚	𐃃	𐃺"));
        assert_eq!(" lttl Phncn: 𐤀	𐤈	𐤔	𐤕",
            disemvowel("A little Phoenician: 𐤀	𐤈	𐤔	𐤕"));
        assert_eq!("W cn hndl mj s wll! 🤣😃👍",
            disemvowel("We can handle emoji as well! 🤣😃👍"))
    }
}