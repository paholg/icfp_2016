extern crate num;
extern crate serde;

use num::Rational;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Point {
    x: Rational,
    y: Rational,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        try!(write!(f, "{},{}", self.x, self.y));
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Polygon {
    vertices: Vec<Point>,
    skeleton: Vec<[usize; 2]>,
}


// A solution is valid iff:
// - All xs and ys of source must be in [0, 1]
// - No repeated coordinates in sources list
// - All facet edges must have length > 0
// - Edges cannot cross (but tips may touch!)
// - All facet polygons are simple (non-intersecting edges)
// - Each source vertex maps to the destination vertex with the same index
// - At source position, the intersection of two different facets has zero area
// - At source position, the union set of all facets exactly matches the initial square
// - The size of the solution is no longer than 5000 Bytes

// A solution is normalized iff:
// - It is valid
// - At source position, if two facets share an edge for a length greater than 0, then the
//   intersection of those facets at destination position must be greater than 0.

// Irrelevent to validity and normality:
// - Skeleton of the original
// - Whether the destinanation points are in [0, 1]
// - Whether the destination can be reached just with folds, or whether it requires parallel
//   transformation and/or rotation
// - Whether the paper is folded at all

pub struct Solution {
    sources: Vec<Point>,
    facets: Vec<Vec<usize>>,
    destinations: Vec<Point>,
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
