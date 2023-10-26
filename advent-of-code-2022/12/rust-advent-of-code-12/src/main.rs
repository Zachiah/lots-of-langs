use pathfinding::prelude::bfs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Point(usize, usize);

#[derive(Hash, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct GraphNode {
    pos: Point,
    height: u8,
    connections: Vec<Point>,
}

struct NodeMatrix(Vec<Vec<GraphNode>>);

impl NodeMatrix {
    fn get_point(&self, p: &Point) -> &GraphNode {
        self.get_point_opt(p).unwrap()
    }

    fn get_point_opt(&self, p: &Point) -> Option<&GraphNode> {
        self.0.get(p.1).and_then(|r| r.get(p.0))
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        [
            if point.0 == 0 {
                None
            } else {
                Some(Point(point.0 - 1, point.1))
            },
            Some(Point(point.0 + 1, point.1)),
            Some(Point(point.0, point.1 + 1)),
            if point.1 == 0 {
                None
            } else {
                Some(Point(point.0, point.1 - 1))
            },
        ]
        .iter()
        .map(|p| p.as_ref().and_then(|point| self.get_point_opt(&point)))
        .map(|x| x.map(|n| n.pos.clone()))
        .flatten()
        .collect()
    }

    fn find_shortest_path(&self, start: &Point, end: &Point) -> Option<Vec<Point>> {
        bfs(
            start,
            |n| self.get_point(n).connections.clone(),
            |n| n == end,
        )
    }
}

fn main() {
    let file = File::open("../data.txt").expect("Failed to read file");

    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    let mut node_matrix = NodeMatrix(
        BufReader::new(file)
            .lines()
            .map(|l| l.expect("Error while reading flie"))
            .enumerate()
            .map(|(y_idx, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x_idx, c)| {
                        let height = match c {
                            'S' => {
                                start = Some(Point(x_idx, y_idx));
                                1
                            }
                            'E' => {
                                end = Some(Point(x_idx, y_idx));
                                26
                            }
                            _ => (c as u8) - 96,
                        };

                        GraphNode {
                            pos: Point(x_idx, y_idx),
                            height,
                            connections: vec![],
                        }
                    })
                    .collect()
            })
            .collect(),
    );

    node_matrix = NodeMatrix(
        node_matrix
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|gn| GraphNode {
                        pos: gn.pos.clone(),
                        height: gn.height,
                        connections: node_matrix
                            .get_neighbors(&gn.pos)
                            .iter()
                            .filter(|p| node_matrix.get_point(p).height < gn.height + 2)
                            .map(|n| n.clone())
                            .collect(),
                    })
                    .collect()
            })
            .collect(),
    );

    let start = start.expect("There should be a start");
    let end = end.expect("There shouold be an end");

    println!(
        "Pt1 Res: {}",
        node_matrix
            .find_shortest_path(&start, &end)
            .map(|path| path.len() - 1)
            .expect("Failed to find path")
    );

    println!(
        "Pt2 Res: {}",
        node_matrix
            .0
            .iter()
            .flatten()
            .filter(|n| n.height == 1)
            .map(|n| n.pos.clone())
            .map(|p| node_matrix.find_shortest_path(&p, &end))
            .flatten()
            .map(|path| path.len() - 1)
            .min()
            .expect("Failed to find any paths")
    );
}
