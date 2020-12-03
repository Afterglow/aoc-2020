const XMOVE: usize = 3;

fn main() {
    let map = std::fs::read_to_string("input.txt").unwrap();
    let mut my_x: usize = 1;
    let width = map.find('\n').unwrap();
    let mut trees = 0;

    println!("00 | {} | START", map.lines().next().unwrap());

    for line in map.lines().skip(1) {
        my_x = my_x + XMOVE;
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
            trees += 1;
        } else {
            let (first, second) = line.split_at(my_x);
            println!("{}X{} | {} MISS", first.strip_suffix('.').unwrap(), second, x_check);
        }
    }

    println!("Hit {} trees", trees);
}
