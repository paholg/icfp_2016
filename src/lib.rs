extern crate num;
extern crate serde;

use num::Rational;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Point {
    x: Rational,
    y: Rational,
}

impl Point {
    pub fn new(x: Rational, y: Rational) -> Point {
        Point { x: x, y: y }
    }

    pub fn in_polygon(&self, polygon: &[Edge]) -> bool {
        let count = polygon.iter().filter(|edge| ray_intersects_segment(&self, &edge)).count();

        match count % 2 {
            0 => false,
            _ => true,
        }
    }
}

fn ray_intersects_segment(p: &Point, seg: &Edge) -> bool {
    let (a, b) = if seg.begin.y < seg.end.y {
        (&seg.begin, &seg.end)
    } else {
        (&seg.end, &seg.begin)
    };

    if p.y < a.y || p.y > b.y {
        return false;
    }

    if p.x > std::cmp::max(a.x.clone(), b.x.clone()) {
        return false;
    }

    if p.x < std::cmp::min(a.x.clone(), b.x.clone()) {
        return true;
    }

    let m_blue = if a.x != p.x {
        (p.y.clone() - a.y.clone()) / (p.x - a.x.clone())
    } else {
        return true;
    };

    let m_red = if a.x.clone() != b.x.clone() {
        (b.y - a.y) / (b.x - a.x)
    } else {
        return false;
    };


    if m_blue >= m_red {
        true
    } else {
        false
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge<T = Point> {
    begin: T,
    end: T,
}

impl<T> Edge<T> {
    pub fn new(begin: T, end: T) -> Edge<T> {
        Edge { begin: begin, end: end }
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        try!(write!(f, "({}, {}) -> ({}, {})", self.begin.x, self.begin.y, self.end.x, self.end.y));

        Ok(())
    }
}

// #[derive(Clone, Debug)]
// pub struct Polygon {
//     pub edges: Vec<Edge>,
// }

// impl Polygon {
//     pub fn new(edges: Vec<Edge>) -> Polygon {
//         Polygon { edges: edges }
//     }
// }

#[derive(Clone, Debug)]
pub struct Problem {
    pub polygons: Vec<Vec<Point>>,
    pub skeleton: Vec<Edge>,
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
            skeleton.push(Edge::new(p1, p2));
        }


        Ok(Problem { polygons: polygons, skeleton: skeleton } )
    }
}

#[derive(Clone, Debug)]
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

fn power_set<T>(items: &mut std::slice::Iter<T>) -> Vec<Vec<T>> where T: Clone {
    let mut power = Vec::new();
    match items.next() {
        None => power.push(Vec::new()),
        Some(item) => {
            for mut set in power_set(items).into_iter() {
                power.push(set.clone());
                set.push(item.clone());
                power.push(set);
            }
        }
    }
    power
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum MaybeKeep {
    Keep,
    Trash,
}
use MaybeKeep::{Keep, Trash};

#[derive(Clone, Debug)]
pub struct Origami {
    pub sources: Vec<Point>,
    pub polygons: Vec<Vec<Edge<usize>>>,
    pub destinations: Vec<Point>,
}

impl Origami {
    pub fn from_problem(problem: Problem) -> Origami {

        let mut destinations = Vec::new();
        for polygon in problem.polygons {
            for vertex in polygon {
                if !destinations.contains(&vertex) {
                    destinations.push(vertex);
                }
            }
        }

        let power: Vec<_> = power_set(&mut problem.skeleton.iter()).into_iter().filter(|e| e.len() > 0).collect();
        let mut to_keep = vec![Trash; power.len()];

        // println!("POWER SET ({} left):", power.len());
        // print_polys(&*power);

        // Get rid of things that aren't polygons. Polygons will contain each point exactly twice.
        for (mut keep, set) in to_keep.iter_mut().zip(power.iter()) {
            let mut points_seen: Vec<(u8, Point)> = Vec::new();
            'set: for edge in set {
                for ref point in &[edge.begin, edge.end] {
                    match points_seen.binary_search_by_key(*point, |&(_, p)| p) {
                        Ok(i) => {
                            points_seen[i].0 += 1;
                            if points_seen[i].0 > 2 {
                                break 'set;
                            }
                        },
                        Err(i) => points_seen.insert(i, (1, **point)),
                    }
                }
            }
            if points_seen.iter().all(|&(n, _)| n == 2) {
                *keep = Keep;
            }
        }

        let polygons: Vec<_> = to_keep.into_iter().zip(power.into_iter()).filter_map(|(to_keep, set)| match to_keep {
            Keep => Some(set),
            Trash => None,
        }).collect();

        // println!("GOT RID OF NON-POLYGONS ({} left):", polygons.len());
        // print_polys(&*polygons);

        // Now get rid of polygons that contain all edges of smaller polygons
        to_keep = vec![Keep; polygons.len()];
        for i in 0..polygons.len() {
            for j in 0..polygons.len() {
                if i != j && to_keep[i] == Keep && to_keep[j] == Keep {
                    to_keep[i] = Trash;
                    for edge in &polygons[j] {
                        if !polygons[i].contains(&edge) {
                            to_keep[i] = Keep;
                            break;
                        }
                    }
                }
            }
        }

        let polygons: Vec<_> = to_keep.into_iter().zip(polygons.into_iter()).filter_map(|(to_keep, polygon)| match to_keep {
            Keep => Some(polygon),
            Trash => None,
        }).collect();

        // println!("GOT RID OF POLYGONS THAT FULLY CONTAIN SMALLER ONES ({} left)", polygons.len());
        // print_polys(&polygons);


        // Finally, get rid of polygons that fully intersect with smaller polygons
        // Does not work if it contains all edges, which is why we do previous thing
        to_keep = vec![Keep; polygons.len()];
        for i in 0..polygons.len() {
            for j in 0..polygons.len() {
                if i != j && to_keep[i] == Keep && to_keep[j] == Keep {
                    for edge in &polygons[i] {
                        // fixme: We hit each point twice. That's kinda dumb.
                        'point: for point in &[edge.begin, edge.end] {
                            for segment in &polygons[j] {
                                if *point == segment.begin || *point == segment.end {
                                    continue 'point;
                                }
                            }

                            if point.in_polygon(&polygons[j]) {
                                to_keep[j] = Trash;
                            }
                        }
                    }
                }
            }
        }

        let polygons: Vec<_> = to_keep.into_iter().zip(polygons.into_iter()).filter_map(|(to_keep, polygon)| match to_keep {
            Keep => Some(polygon),
            Trash => None,
        }).collect();

        // Really finally, we'll make sure one doesn't contain all the vertices of another
        to_keep = vec![Keep; polygons.len()];
        for i in 0..polygons.len() {
            let poly_iter = polygons[i].iter().flat_map(|edge| Some(edge.begin).into_iter().chain(Some(edge.end).into_iter()));
            let poly: Vec<_> = poly_iter.collect();
            'jloop: for j in 0..polygons.len() {
                if i != j && to_keep[i] == Keep && to_keep[j] == Keep {
                    to_keep[i] = Trash;
                    for edge in &polygons[j] {
                        for point in &[edge.begin, edge.end] {
                            if !poly.contains(&&point) {
                                to_keep[i] = Keep;
                                break 'jloop;
                            }
                        }
                    }
                }
            }
        }

        let polygons: Vec<_> = to_keep.into_iter().zip(polygons.into_iter()).filter_map(|(to_keep, polygon)| match to_keep {
            Keep => Some(polygon),
            Trash => None,
        }).collect();

        // println!("GOT RID OF POLYGONS CONTAINING POINTS FROM OTHERS ({} left)", polygons.len());
        // print_polys(&polygons);

        let poly_indices: Vec<_> = polygons.into_iter().map(|poly| {
            poly.into_iter().map(|edge| {
                let begin = destinations.iter().position(|&p| p == edge.begin).unwrap();
                let end = destinations.iter().position(|&p| p == edge.end).unwrap();
                Edge::new(begin, end)
            }).collect()
        }).collect();

        Origami {
            sources: destinations.clone(),
            polygons: poly_indices,
            destinations: destinations,
        }
    }
}

fn print_polys(polys: &[Vec<Edge>]) {
    for poly in polys {
        println!("Polygon:");
        for edge in poly {
            println!("{}", edge);
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
        // for p in &self.sources {
        //     if p.x < Rational::new(0, 1) || p.x > Rational::new(1, 1)
        //         || p.y < Rational::new(0, 1) || p.y > Rational::new(1, 1) {
        //             return Err(SolError::OutOfBounds);
        //         }
        // }

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

    pub fn from_origami(origami: Origami) -> Solution {
        let new_polys = origami.polygons.into_iter().map(|poly| {
            let mut points: Vec<usize> = Vec::with_capacity(poly.len());
            for edge in poly {
                if points.iter().find(|&&p| p == edge.begin) == None {
                    points.push(edge.begin);
                }
                if points.iter().find(|&&p| p == edge.end) == None {
                    points.push(edge.end);
                }
            }
            points
        }).collect();

        Solution {
            sources: origami.sources,
            facets: new_polys,
            destinations: origami.destinations,
        }
    }
}
