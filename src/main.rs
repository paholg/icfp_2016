extern crate icfp_2016 as icfp;
extern crate num;

use icfp::*;

fn main() {
    // let sol: Solution = Solution {
    //     sources: vec![
    //         Point::new(Rational::new(0, 1), Rational::new(0, 1)),
    //         Point::new(Rational::new(1, 1), Rational::new(0, 1)),
    //         Point::new(Rational::new(1, 1), Rational::new(1, 1)),
    //         Point::new(Rational::new(0, 1), Rational::new(1, 1)),
    //         Point::new(Rational::new(0, 1), Rational::new(1, 2)),
    //         Point::new(Rational::new(1, 2), Rational::new(1, 2)),
    //         Point::new(Rational::new(1, 2), Rational::new(1, 1)),
    //     ],
    //     facets: vec![
    //         vec![0, 1, 5, 4],
    //         vec![1, 2, 6, 5],
    //         vec![4, 5, 3],
    //         vec![5, 6, 3],
    //     ],
    //     destinations: vec![
    //         Point::new(Rational::new(0, 1), Rational::new(0, 1)),
    //         Point::new(Rational::new(1, 1), Rational::new(0, 1)),
    //         Point::new(Rational::new(0, 1), Rational::new(0, 1)),
    //         Point::new(Rational::new(0, 1), Rational::new(0, 1)),
    //         Point::new(Rational::new(0, 1), Rational::new(1, 2)),
    //         Point::new(Rational::new(1, 2), Rational::new(1, 2)),
    //         Point::new(Rational::new(0, 1), Rational::new(1, 2)),
    //     ],
    // };

    // assert!(sol.verify().is_ok());

    let mut args = std::env::args();

    let fname = args.nth(1).unwrap();

    let f = std::fs::File::open(fname).unwrap();
    let reader = std::io::BufReader::new(f);
    let problem = Problem::read(reader).unwrap();
    let origami = Origami::from_problem(problem);

    let sol = Solution::from_origami(origami);

    match sol.verify() {
        Err(e) => { println!("Couldn't verify solution: {:?}", e);
                    std::process::exit(1);
        }
        Ok(()) => (),
    }

    println!("{}", sol);
}
