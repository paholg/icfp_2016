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

#[derive(Clone, Debug)]
pub struct Polygon {
    pub vertices: Vec<Point>,
    pub skeleton: Vec<[usize; 2]>,
}

impl Polygon {
    pub fn new() -> Polygon {
        Polygon {
            vertices: Vec::new(),
            skeleton: Vec::new(),
        }
    }

    pub fn read<B: std::io::BufRead>(r: &mut B) -> Result<Polygon, std::io::Error> {
        let mut buffer = String::new();
        try!(r.read_line(&mut buffer));

        use std::io::{Error, ErrorKind};
        let n_polys: usize = try!(buffer.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "parse error")));

        let mut vertices = Vec::with_capacity(n_polys);

        Ok(polygon)
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

pub enum SolError {
    OutOfBounds,
    Duplicates,
    FacetEdgeLength0,
}

impl Solution {
    pub fn from_polygon(polygon: Polygon) -> Solution {
        let len = polygon.vertices.len();

        Solution {
            sources: polygon.vertices,
            facets: Vec::new(),
            destinations: Vec::with_capacity(len)
        }
    }

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
