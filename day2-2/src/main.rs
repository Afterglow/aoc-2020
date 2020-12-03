use regex::Regex;

fn main() {
    let mut ok: usize = 0;
    let file = std::fs::read_to_string("input.txt").unwrap();

    // 6-10 g: rhtwpsxrzgpgxhk
    let re = Regex::new(r"^(?P<from>[0-9]+)-(?P<to>[0-9]+) (?P<needle>\w): (?P<haystack>\S+)$")
               .unwrap();
    for line in file.lines() {
        let captures = match re.captures(line) {
            Some(captures) => captures,
            None => {
                panic!("No match found for regex in '{}'", line)
            }
        };
        
        // Minus one on indices beacuse policy is no zero indexed and array is
        let first: usize = captures.name("from").unwrap().as_str().parse::<usize>().unwrap() - 1;
        let second: usize = captures.name("to").unwrap().as_str().parse::<usize>().unwrap() - 1;
        let needle: char = captures.name("needle").unwrap().as_str().parse().unwrap();
        let haystack = captures.name("haystack").unwrap().as_str().as_bytes();

        // Both characters cannot match _and_ be the same so if they're the same its wrong
        if haystack[first] == haystack[second] {
            continue
        }

        if haystack[first] == needle as u8 || haystack[second] == needle as u8 {
            ok += 1;
        }
    }
    println!("Found {} matches", ok);
}