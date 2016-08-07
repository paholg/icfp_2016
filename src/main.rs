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

    let arg = args.nth(1).unwrap();
    let fname = std::path::Path::new(&arg);

    let f = std::fs::File::open(fname).unwrap();
    let reader = std::io::BufReader::new(f);
    let problem = Problem::read(reader).unwrap();
    let origami = Origami::from_problem(problem);

    let solution = Solution::from_origami(origami);

    match solution.verify() {
        Err(e) => { println!("Couldn't verify solution: {:?}", e);
                    println!("Failed to solve: {:?}", fname);
                    std::process::exit(1);
        }
        Ok(()) => (),
    }

    let mut outname = std::path::PathBuf::new();
    outname.push("solutions/");
    outname.push(fname.file_name().unwrap());
    let mut f = std::fs::File::create(&outname).unwrap();

    use std::io::Write;
    write!(f, "{}", solution).unwrap();

    println!("Solved: {:?}", fname);
}
