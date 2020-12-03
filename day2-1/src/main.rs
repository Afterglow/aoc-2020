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
        let from: usize = captures.name("from").unwrap().as_str().parse().unwrap();
        let to: usize = captures.name("to").unwrap().as_str().parse().unwrap();
        let needle: char = captures.name("needle").unwrap().as_str().parse().unwrap();
        let haystack = captures.name("haystack").unwrap().as_str();

        let count = haystack.matches(needle).count();

        if count >= from && count <= to {
            ok += 1;
        }
    }
    println!("Found {} matches", ok);
}