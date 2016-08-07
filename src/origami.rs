use {Problem, Point, Edge, power_set};
use MaybeKeep::*;


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

        for edge in problem.skeleton.iter() {
            for vertex in &[edge.begin, edge.end] {
                if !destinations.contains(vertex) {
                    destinations.push(*vertex);
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

        // map from edges being defined by points to defined by indices
        let poly_indices: Vec<_> = polygons.into_iter().map(|poly| {
            poly.into_iter().map(|edge| {
                let begin = destinations.iter().position(|&p| p == edge.begin).expect("begin");
                let end = destinations.iter().position(|&p| p == edge.end).expect("end");
                Edge::new(begin, end)
            }).collect()
        }).collect();

        Origami {
            sources: destinations.clone(),
            polygons: poly_indices,
            destinations: destinations,
        }
    }

    fn unfold(&mut self) {
    }
}

pub fn reflect(facet: &[Edge], line: Edge) -> Vec<Edge> {
    let mut out = Vec::with_capacity(facet.len());
    for edge in facet {
        let begin = edge.begin.reflect(&line);
        let end = edge.end.reflect(&line);

        out.push(Edge::new(begin, end));
    }
    out
}
