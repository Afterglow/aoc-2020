const XMOVE: [usize; 5] = [1, 3, 5, 7, 1];
const YMOVE: [usize; 5] = [1, 1, 1, 1, 2];

fn main() {
    let map = std::fs::read_to_string("input.txt").unwrap();
    let mut my_x: usize = 1;
    let width = map.find('\n').unwrap();
    let mut trees: [usize; 5] = [0; 5];
    let mut offset = 0;

    while offset < XMOVE.len() {
        let mut mytrees = 0;
        let mut lines = map.lines();
        println!("00 | {} | START", lines.next().unwrap()); // print and discard first line

        loop {
            let line = match lines.nth(YMOVE[offset]-1) {
                Some(line) => line,
                None => break
            };

            my_x = my_x + XMOVE[offset];
            print!("{:02} | ", my_x);
            if my_x > width {
                my_x = my_x % width;
            }

            let x_check = match my_x {
                0 => 0,
                _ => my_x - 1
            };

            if line.as_bytes()[x_check] == '#' as u8 {
                let (first, second) = line.split_at(my_x);
                println!("{}X{} | {} HIT", first.strip_suffix('#').unwrap(), second, x_check);
                mytrees += 1;
            } else {
                let (first, second) = line.split_at(my_x);
                println!("{}0{} | {} MISS", first.strip_suffix('.').unwrap(), second, x_check);
            }
        }
        trees[offset] = mytrees;
        println!("Hit {} trees this loop", mytrees);
        offset += 1;
        my_x = 1;
    }
    println!("Hit {} trees", trees.iter().sum::<usize>());
    println!("The answer is {} trees", trees.iter().product::<usize>());
}
