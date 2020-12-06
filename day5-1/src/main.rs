const MAX_SEATS: usize = 1024;

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut seats: [u8; MAX_SEATS] = [0; MAX_SEATS];

    let mut smax = 0;
    let mut my_seat: usize = 0;

    for line in file.lines() {
        // Replace R/L/F/B with 1s and 0s
        let mut line = line.replace('L', "0");
        line = line.replace('R', "1");
        line = line.replace('F', "0");
        line = line.replace('B', "1");

        let (rows, columns) = line.split_at(7);
        println!("R: {} | C: {}", rows, columns);

        let column = usize::from_str_radix(columns, 2).unwrap();
        let row = usize::from_str_radix(rows, 2).unwrap();
        // let seat = row * 8 + column;
        let seat = usize::from_str_radix(line.as_str(), 2).unwrap();

        println!("S: {} > R: {} | C: {}", seat, row, column);
        seats[seat] = 1;

        if seat > smax {
            smax = seat;
        }
    }
    let mut occ: bool = false;
    for s in 0..MAX_SEATS {
        if occ && seats[s] == 0 {
            my_seat = s;
            break;
        } else if seats[s] == 1 {
            occ = true;
        } else {
            occ = false;
        }
    }
    println!("{:?}", seats);
    println!("Maximum seat is: {}", smax);
    println!("My seat is: {}", my_seat);
}
