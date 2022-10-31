use std::cmp::Ordering;
use std::cmp;
use itertools::{sorted, izip};
use itertools::Itertools;

use std::fs::File;
use std::io::LineWriter;


const SIZE: usize = 256;
const SAVEPATH: &'static str = "/tmp/test.txt";


#[derive(Copy, Clone, Debug, Eq)]
struct Edge {
    i: u8,
    j: u8,
}

impl Edge {
    fn new(x: u8, y: u8) -> Edge {
        Edge {i: cmp::min(x, y), j: cmp::max(x, y)}
    }
}

impl PartialEq for Edge {
    fn eq(&self, e: &Edge) -> bool {
        // why not just use if statements?
        // hrmm. == w pointers?!
        let a: bool = match self.i.cmp(&e.i) {
            Ordering::Equal => true,
            _ => false,
        };
        let b: bool = match self.j.cmp(&e.j) {
            Ordering::Equal => true,
            _ => false,
        };
        a & b
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, e: &Edge) -> Ordering {
        match self.i.cmp(&e.i) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                match self.j.cmp(&e.j) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => Ordering::Equal,
                }
            }
        }

    }
}

#[derive(Copy, Clone, Debug, Eq)]
struct Graph {
    // vecs use heap memory

    // edge_list: Vec<u8>,
    edge_list: [Edge; SIZE],

    // arrays use stack memory
    // but. need to pick a size if we use arrays.

    n_nodes: u8,
    n_edges: u8,
    max_edge: Edge,
}

impl Graph {
    fn new(edge_list: Vec<Edge>, n_nodes: u8) -> Graph {
        let smallest_edge = Edge {i: 0, j: 0};
        let mut edge_list_internal = [smallest_edge; SIZE];
        let n = edge_list.len();
        // should sort it here?
        for i in 0..n {
            edge_list_internal[i] = edge_list[i];
        }
        Graph {edge_list: edge_list_internal, n_nodes: n_nodes, n_edges: edge_list.len() as u8, max_edge: *edge_list_internal.iter().max().unwrap()}
    }

    fn append(&mut self, e: Edge) {
        // this appended edge should be larger
        assert_eq!(Edge::lt(&self.max_edge, &e), true);

        self.edge_list[self.n_edges as usize] = e;
        self.n_edges = self.n_edges + 1;
        self.max_edge = e;
    }

}

fn permute(g: &Graph, perm: Vec<u8>) -> Graph {
    let mut edge_list: Vec<Edge> = Vec::new();
    for k in 0..g.n_edges {
        let (i, j) = (g.edge_list[k as usize].i, g.edge_list[k as usize].j);
        let (i_, j_) = (perm[i as usize], perm[j as usize]);
        let permuted_edge = Edge::new(i_, j_);
        edge_list.push(permuted_edge);
    }
    return Graph::new(edge_list, g.n_nodes.clone())
}

impl PartialEq for Graph {
    fn eq(&self, g: &Graph) -> bool {
        let mut result: bool = true;
        for (e1, e2) in izip!(sorted(self.edge_list), sorted(g.edge_list)) {
            if e1 != e2 {
                result = false;
            }
        }
        return result
    }
}

impl PartialOrd for Graph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Graph {
    fn cmp(&self, g: &Graph) -> Ordering {
        // assumes edges are already sorted
        let mut result = Ordering::Equal;
        for (e1, e2) in izip!(self.edge_list.iter(),g.edge_list.iter()) {

            if e1 < e2 {
                result = Ordering::Less;
                break
            } else if e1 == e2 {
                continue
            } else if e1 > e2 {
                result = Ordering::Greater;
                break
            }

            // if we are just checking the smallest edges then stop
            if (e1 == &Edge::new(0,0)) & (e2 == &Edge::new(0,0)) {
                break
            }
        }

        if result == Ordering::Equal {
            if self.n_edges < g.n_edges {
                result = Ordering::Less;
            } else if self.n_edges > g.n_edges {
                result = Ordering::Greater;
            }
        }
        return result
    }
}

fn is_max_edge(e: &Edge, n: &u8) -> bool {
    if (e.i >= n-1) & (e.j >= *n) {true}
    else {false}
}

fn is_canonical(g: &Graph) -> bool {
    // println!("sdgfdfs");
    for perm in Itertools::permutations((0..g.n_nodes), (g.n_nodes) as usize) {
        // dont need to both w the first perm. [0,1,2,3,4,5...]

        // let g_ = g.permute(perm);
        let g_ = permute(g, perm);
        // let g_ = g.clone();
        if Graph::lt(&g_, &g) {
            return false
        }
    }
    return true
}

fn does_satisfy_constraints(g: &Graph) -> bool {
    // return g.n_edges == 3
    return false
}

fn could_satisfy_constraints(g: &Graph) -> bool {
    return true
}

fn write_to_file(g: &Graph) {
    let mut edge_list: Vec<Edge> = Vec::new();
    for i in 0..g.n_edges {
        edge_list.push(g.edge_list[i as usize]);
        // s += str(g.edge_list[i as usize]);
    }
    let s = format!("{:?}", edge_list);
    println!("{}", s);

    // let file = File::create(SAVEPATH);
    // let mut file = LineWriter::new(file);

}

fn successor(e: &Edge, n: &u8) -> Option<Edge> {
    if e.j < *n {
        return Some(Edge::new(e.i, e.j+1))
    } else if (e.j == *n) & (e.i < n-1) {
        return Some(Edge::new(e.i+1, e.i+2))
    } else {return None}
}

fn graphgen(g: Graph, n: u8) {
    // how to control the order that the graphs are explored?
    // does this work depth first?
    if is_canonical(&g) {
        if does_satisfy_constraints(&g) {
            write_to_file(&g);
        } else if could_satisfy_constraints(&g) {
            let mut current_edge = g.max_edge.clone();
            while let Some(e) = successor(&current_edge, &(n-1)) {
                let mut g_ = g.clone();
                g_.append(e);
                graphgen(g_, n);
                write_to_file(&g_);
                current_edge = e;
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edges_lt() {
        let e1 = Edge::new(1, 2);
        let e2 = Edge::new(2, 3);
        assert_eq!(Edge::lt(&e1, &e2), true);
        assert_eq!(Edge::lt(&e2, &e1), false);
    }

    #[test]
    fn test_max_edge_list() {
        let e1 = Edge::new(1, 2);
        let e2 = Edge::new(2, 3);
        let e3 = Edge::new(8, 5);
        let e4 = Edge::new(7, 6);

        let edge_list = [e1, e2, e3, e4];

        assert_eq!(edge_list.iter().max().unwrap(), &e4);
    }

    #[test]
    fn test_sorted_edge_list() {
        let e1 = Edge::new(1, 2);
        let e2 = Edge::new(2, 3);
        let e3 = Edge::new(8, 5);
        let e4 = Edge::new(7, 6);

        let edge_list = [e1, e2, e3, e4];

        // assert_eq!(sorted(edge_list), edge_list.iter());
    }

    #[test]
    fn test_new_graph() {
        let e1 = Edge::new(1, 2);
        let e2 = Edge::new(2, 3);
        let e3 = Edge::new(8, 5);
        let e4 = Edge::new(7, 6);

        let edge_list = vec![e1, e2, e3, e4];
        let g = Graph::new(edge_list, 3);

        assert_eq!(g.edge_list[0], e1);
        assert_eq!(g.max_edge, e4);
    }

    #[test]
    fn test_graph_lt() {
        // should generate 100 rnd graphs and check they can be ordered!?
        let g1 = Graph::new(vec![Edge::new(1, 2), Edge::new(1, 3)], 5);
        let g2 = Graph::new(vec![Edge::new(2, 3), Edge::new(2, 4)], 5);
        let g3 = Graph::new(vec![Edge::new(2, 3), Edge::new(2, 4), Edge::new(3, 5)], 5);

        assert_eq!(Graph::lt(&g1, &g2), true);
        assert_eq!(Graph::lt(&g2, &g1), false);
        assert_eq!(Graph::lt(&g2, &g3), true);
    }

    #[test]
    fn test_graph_append() {
        let e1 = Edge::new(1, 2);
        let e2 = Edge::new(1, 3);
        let mut g = Graph::new(vec![e1], 5);

        assert_eq!(g.max_edge, e1);
        assert_eq!(g.n_edges, 1);

        g.append(e2);
        assert_eq!(g.max_edge, e2);
        assert_eq!(g.n_edges, 2);
        assert_eq!(g.edge_list[0], e1);
        assert_eq!(g.edge_list[1], e2);
    }

    #[test]
    fn test_next_edge() {
        let e1 = Edge::new(1, 2);
        let e2 = successor(&e1, &5).unwrap();

        assert_eq!(Edge::lt(&e1, &e2), true);
        assert_eq!(Edge::lt(&e2, &e1), false);
    }

    #[test]
    fn test_permute() {
        let e = Edge::new(3, 4);
        let g1 = Graph::new(vec![e], 6);
        let g2 = permute(&g1, vec![5,4,3,2,1,0]);
        println!("{:?}", g2);

    }
}

fn main() {
    let n = 3;
    let g = Graph::new(vec![Edge::new(0, 1)], n);
    graphgen(g, n);
}
