extern crate num;
extern crate serde;

use num::Rational;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Point {
    x: Rational,
    y: Rational,
}

impl Point {
    pub fn new(x: Rational, y: Rational) -> Point {
        Point { x: x, y: y }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        try!(write!(f, "{},{}", self.x, self.y));
        Ok(())
    }
}

impl std::str::FromStr for Point {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x: Rational = try!(try!(iter.next().ok_or(std::fmt::Error)).parse().map_err(|_| std::fmt::Error));
        let y: Rational = try!(try!(iter.next().ok_or(std::fmt::Error)).parse().map_err(|_| std::fmt::Error));
        Ok(Point::new(x, y))
    }
}

// #[derive(Clone, Debug)]
// pub struct Polygon {
//     pub vertices: Vec<Point>
// }

// impl Polygon {
//     pub fn new(vertices: Vec<Point>) -> Polygon {
//         Polygon { vertices: vertices }
//     }
// }

#[derive(Clone, Debug)]
pub struct Problem {
    pub polygons: Vec<Vec<Point>>,
    pub skeleton: Vec<[Point; 2]>,
}

impl Problem {
    pub fn new() -> Problem {
        Problem {
            polygons: Vec::new(),
            skeleton: Vec::new(),
        }
    }

    pub fn read<B: std::io::BufRead>(r: B) -> Result<Problem, std::io::Error> {
        let mut lines = r.lines();

        use std::io::{Error, ErrorKind};
        let line = try!(lines.next().unwrap());
        let npolys: usize = try!(line.parse()
                                 .map_err(|_| Error::new(ErrorKind::InvalidInput, "parse error: npolys")));

        let mut polygons = Vec::with_capacity(npolys);

        for _ in 0..npolys {
            let line = try!(lines.next().unwrap());
            let nvertices: usize = try!(line.parse()
                                        .map_err(|_| Error::new(ErrorKind::InvalidInput, "parse error: nvertices")));
            let mut polygon = Vec::with_capacity(nvertices);
            for _ in 0..nvertices {
                let line = try!(lines.next().unwrap());
                let vertex: Point = try!(line.parse()
                                         .map_err(|_| Error::new(ErrorKind::InvalidInput, "parse error: vertex")));
                polygon.push(vertex);
            }
            polygons.push(polygon);
        }

        let line = try!(lines.next().unwrap());
        let nedges: usize = try!(line.parse()
                                 .map_err(|_| Error::new(ErrorKind::InvalidInput, "parse error: nedges")));

        let mut skeleton = Vec::with_capacity(nedges);
        for _ in 0..nedges {
            let line = try!(lines.next().unwrap());
            let mut points = line.split_whitespace();
            let p1: Point = try!(
                try!(points.next().ok_or(Error::new(ErrorKind::InvalidInput, "parse error: p1")))
                    .trim().parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "parse error: p1")));
            let p2: Point = try!(
                try!(points.next().ok_or(Error::new(ErrorKind::InvalidInput, "parse error: p2")))
                    .trim().parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "parse error: p2")));
            skeleton.push([p1, p2]);
        }


        Ok(Problem { polygons: polygons, skeleton: skeleton } )
    }
}

pub struct Solution {
    pub sources: Vec<Point>,
    pub facets: Vec<Vec<usize>>,
    pub destinations: Vec<Point>,
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        try!(write!(f, "{}\n", self.sources.len()));
        for source in &self.sources {
            try!(write!(f, "{}\n", source));
        }

        try!(write!(f, "{}\n", self.facets.len()));

        for facet in &self.facets {
            try!(write!(f, "{}", facet.len()));
            for vertex in facet {
                try!(write!(f, " {}", vertex));
            }
            try!(write!(f, "\n"));
        }

        for destination in &self.destinations {
            try!(write!(f, "{}\n", destination));
        }

        Ok(())
    }
}

impl Solution {
    pub fn from_problem(problem: Problem) -> Solution {

        let mut destinations = Vec::new();
        for polygon in problem.polygons {
            for vertex in polygon {
                if !destinations.contains(&vertex) {
                    destinations.push(vertex);
                }
            }
        }

        let skel = problem.skeleton;

        let mut facets: Vec<Vec<usize>> = Vec::new();

        for vertex in &destinations {
            
        }

        Solution {
            sources: destinations.clone(),
            facets: facets,
            destinations: destinations,
        }

    }
}

#[derive(Copy, Clone, Debug)]
pub enum SolError {
    OutOfBounds,
    Duplicates,
    FacetEdgeLength0,
}

impl Solution {
    /// Verifies whether a solution is valid.
    ///
    /// A solution is valid iff:
    /// - All xs and ys of source must be in [0, 1]
    /// - No repeated coordinates in sources list
    /// - All facet edges must have length > 0
    /// - Edges cannot cross (but tips may touch!)
    /// - All facet polygons are simple (non-intersecting edges)
    /// - Each source vertex maps to the destination vertex with the same index
    /// - At source position, the intersection of two different facets has zero area
    /// - At source position, the union set of all facets exactly matches the initial square
    /// - The size of the solution is no longer than 5000 Bytes

    /// A solution is normalized iff:
    /// - It is valid
    /// - At source position, if two facets share an edge for a length greater than 0, then the
    ///   intersection of those facets at destination position must be greater than 0.

    /// Irrelevent to validity and normality:
    /// - Skeleton of the original
    /// - Whether the destinanation points are in [0, 1]
    /// - Whether the destination can be reached just with folds, or whether it requires parallel
    ///   transformation and/or rotation
    /// - Whether the paper is folded at all

    pub fn verify(&self) -> Result<(), SolError> {
        // Check source points are in bounds:
        for p in &self.sources {
            if p.x < Rational::new(0, 1) || p.x > Rational::new(1, 1)
                || p.y < Rational::new(0, 1) || p.y > Rational::new(1, 1) {
                return Err(SolError::OutOfBounds);
            }
        }

        // Ensure no repeated coordinates:
        let mut sorted = self.sources.clone();
        sorted.sort();
        sorted.dedup();

        if self.sources.len() != sorted.len() {
            return Err(SolError::Duplicates);
        }

        // TODO: Finish
        // // Ensure facet edges have length > 0:
        // for facet in &self.facets {

        // }

        Ok(())
    }
}
