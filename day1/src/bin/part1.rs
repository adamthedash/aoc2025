use day1::parse_input;

fn main() {
    let inputs = parse_input();

    let cumsum = inputs.scan(50, |acc, x| {
        *acc += x;
        *acc = acc.rem_euclid(100);
        Some(*acc)
    });

    let answer = cumsum.filter(|x| *x == 0).count();
    println!("{answer}");
}
