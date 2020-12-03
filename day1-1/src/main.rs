use std::str::FromStr;

const TARGET: usize = 2020;

fn main() {
    let mut numbers = read_lines::<usize>("input.txt");
    // Put them into ascending order
    numbers.sort_unstable();
    // Flip it into descending order
    numbers.reverse();

    'main: loop {
        let first_number = match numbers.pop() {
            Some(first_number) => first_number,
            None => break,
        };

        for number in &numbers {
            let result = number + first_number;
            if  result == TARGET {
              println!("Found {} + {} = {}", first_number, number, TARGET);
              println!("{}", first_number * number);
              break 'main;
            }

            // Because the list is in descending numerical order if we are
            // smaller than the required result there is no point in continuing
            if result < TARGET {
                break;
            }
        }
    }
}

fn read_lines<T: FromStr>(file: &str) -> Vec<usize> {
    std::fs::read_to_string(file)
      .unwrap()
      .lines()
      .map(|x| x.parse().unwrap())
      .collect()
}
