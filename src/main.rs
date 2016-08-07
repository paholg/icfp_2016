extern crate icfp_2016 as icfp;
extern crate num;

use num::Rational;
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

    let mut f = std::fs::File::open("problems/sample.txt").unwrap();
    let mut reader = std::io::BufReader::new(f);
    let problem = Problem::read(reader).unwrap();
    let solution = Solution::from_problem(problem);

    solution.verify().unwrap();

    println!("{}", solution);
}
