fn main() {
    let mut model = Model { first: Vec::new(), second: Vec::new(), third: Vec::new() };
    let n = 7;
    for i in 1..n {
        model.first.push(n - i);
    }
    println!("({:?}, {:?}, {:?})", model.first, model.second, model.third);
    let mut moves: u8 = 0;
    solve(&mut model.first, &mut model.third, &mut model.second, n - 1, &mut moves);
    println!("({:?}, {:?}, {:?})", model.first, model.second, model.third);
    println!("{:?}", moves);
}

struct Model {
    first: Vec<u8>,
    second: Vec<u8>,
    third: Vec<u8>,
}

fn check_invariant(arg: &mut Vec<u8>) -> () {
    if arg.len() > 1 {
        for i in 0..arg.len() - 2 {
            assert!(arg[i] > arg[i + 1])
        }
    }
}

fn solve(source: &mut Vec<u8>, destination: &mut Vec<u8>, aux: &mut Vec<u8>, n: u8, moves: &mut u8) -> () {
    check_invariant(source);
    check_invariant(destination);
    check_invariant(aux);
    if n == 2 {
        aux.push(source.pop().unwrap());
        destination.push(source.pop().unwrap());
        destination.push(aux.pop().unwrap());
        *moves = *moves + 3
    } else {
        solve(source, aux, destination, n - 1, moves);
        destination.push(source.pop().unwrap());
        *moves = *moves + 1;
        solve(aux, destination, source, n - 1, moves);
    }
}