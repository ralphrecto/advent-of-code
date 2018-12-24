use fileutil;
use std::ascii::AsciiExt;

fn reacts(c1: char, c2: char) -> bool {
    return c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2;
}

pub fn run() -> () {

    match fileutil::read_file("./data/05.txt") {
        Ok(mut orig) => {
            let orig_initial_len = orig.len();
            let mut num_total_reactions = 0;

            loop {
                let copy: String = orig.clone();

                let mut found_reaction = false;
                let mut prev_char: Option<char> = Option::None;

                let chars = copy.chars();
                for (idx, c) in chars.enumerate() {
                    match prev_char {
                        Some(prev) => {
                            if reacts(prev, c) {
                                // Mutate the original, eliding borrow errors here.
                                orig.drain(idx-1..idx+1);
                                found_reaction = true;
                                num_total_reactions += 1;

                                // TODO: optimization is possible here by continuing on to mutate
                                // orig. However, idx will need to be modified to reflect the
                                // reactions that have already been drained from orig.
                                break;
                            }
                        }
                        None => { }
                    }

                    prev_char = Option::Some(c);
                }

                if (!found_reaction) {
                    break;
                }
            }

            println!(
                "total reactions made: {}\norig len: {}\nfinal len: {}",
                num_total_reactions,
                orig_initial_len,
                orig.len()
            );
        }
        Err(e) => println!("oh no!")
    }
}
