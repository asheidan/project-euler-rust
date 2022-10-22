use std::env;

/// Solution for lattice paths from project Euler
///
/// https://projecteuler.net/problem=15
///
/// How many paths are there throu a grid of a certain size if you can only move right and down.
///
/// I'm quite pleased with this solution.
///
/// My idea here is based on that in each "node" we can choose between a certain paths, those paths
/// in turn branch out themselves. Starting at the goal there is a single path:
///
///     1
///
/// In the previous nodes there are different numbers based an where they are. The current nodes
/// number of paths is equal to the sum of the paths one can choose from there
///
///     2 - 1
///     |   |
///     1 - 1
///
/// Since this is symmetrical it can be stored in a triangular matrix. (Reversing it just to make
/// the loops nicer.
///
///     1
///     1 2
///     1 3 6
///
/// Since we are only really ever calculating a single row at a time it's enough to only store one
/// row chich is long enough to fit the final row (size + 1).
///
/// Then we can just iterator through row by row and add the possible paths reachable from each
/// position and in the end the final position in the Vec should be the number of paths through the
/// lattice.
fn main() {
    let args: Vec<String> = env::args().collect();

    let size = args.get(1).unwrap().parse::<i32>().unwrap();

    println!("size: {:?}", size);

    let mut matrix: Vec<u64> = (0..=size).map(|_| 1).collect();

    for y in 1..=size {
        for x in 1..y {
            let i = x as usize;
            let new_value = matrix[i] + matrix[i - 1];
            //eprint!("|{:>6}", new_value);
            matrix[i] = new_value;
        }

        let i = y as usize;
        let new_value = matrix[i - 1] * 2;
        //eprint!("|{:>6}", new_value);
        matrix[i] = new_value;

        eprintln!("");
    }

    println!("{}", matrix.pop().unwrap());
}
