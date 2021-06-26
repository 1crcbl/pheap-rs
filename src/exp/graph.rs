use std::collections::HashMap;

use num_traits::{Bounded, Num, Zero};

use crate::PairingHeap;

/// An undirected graph.
///
/// # Examples
/// The following example shows how to construct a graph and find the shortest path between node 1 and 5.
/// The data is taken from the illustration in Wikipedia's page for [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
/// ```
/// use pheap::graph::Graph;
///
/// let mut g = Graph::<u32>::with_size(6, 8);
///
/// g.add_weighted_edges(0, 1, 7);
/// g.add_weighted_edges(0, 2, 9);
/// g.add_weighted_edges(0, 5, 14);
/// g.add_weighted_edges(1, 2, 10);
/// g.add_weighted_edges(1, 3, 15);
/// g.add_weighted_edges(2, 5, 2);
/// g.add_weighted_edges(2, 3, 11);
/// g.add_weighted_edges(3, 4, 6);
/// g.add_weighted_edges(4, 5, 9);
///
/// let mut sp = g.sssp_dijkstra(0, Some(&[4]));
/// assert_eq!(1, sp.len());
///
/// let sp = sp.pop().unwrap();
/// assert_eq!(20, sp.dist());
/// assert_eq!(&[0, 2, 5, 4], sp.path().as_slice());
///
/// ```
#[derive(Debug)]
pub struct Graph<W> {
    n_nodes: usize,
    n_edges: usize,
    weights: HashMap<usize, Vec<(usize, W)>>,
}

impl<W> Graph<W> {
    /// Creates an empty graph given the number of nodes and edges.
    pub fn with_size(n_nodes: usize, n_edges: usize) -> Self {
        Self {
            n_nodes,
            n_edges,
            weights: HashMap::new(),
        }
    }

    /// Adds a weighted edge to the graph.
    ///
    /// If the edge already exists in the graph, the weight will be updated.
    pub fn add_weighted_edges(&mut self, node1: usize, node2: usize, weight: W)
    where
        W: Clone + Copy,
    {
        if node1 != node2 {
            self.insert_weight(node1, node2, weight);
            self.insert_weight(node2, node1, weight);
        }
    }

    /// Returns the neighbours of a node.
    fn neighbours(&self, node: &usize) -> Option<&Vec<(usize, W)>> {
        self.weights.get(&node)
    }

    /// Finds the shortest paths from a source node to destination nodes.
    ///
    /// If no destination nodes are given, shortest paths for all nodes in the graph will be returned.
    /// In this case, the final result might be huge for large graph and thus consumes a lot of memory.
    ///
    /// If you want to keep the result for later usage and want to save memory, consider using
    /// the lazy version [`Graph::sssp_dijkstra_lazy`].
    pub fn sssp_dijkstra(&self, src: usize, dest: Option<&[usize]>) -> Vec<ShortestPath<W>>
    where
        W: Bounded + Num + Zero + PartialOrd + Copy,
    {
        let nodes = self.dijkstra(src);

        let result = match dest {
            Some(dst) => {
                // Repeated code.
                let mut result = Vec::with_capacity(dst.len());

                for ii in dst {
                    let end_node = &nodes[*ii];
                    let expected = end_node.len + 1;

                    let mut len = 0;
                    let mut chain = Vec::with_capacity(expected);
                    chain.push(*ii);
                    let mut next = end_node.pred;

                    while len < expected {
                        chain.insert(0, next);
                        next = nodes[next].pred;
                        len = chain.len();
                    }

                    result.push(ShortestPath {
                        src: src,
                        dest: *ii,
                        dist: end_node.dist,
                        path: chain,
                    });
                }

                result
            }
            None => {
                // Repeated code.
                let mut result: Vec<ShortestPath<W>> = Vec::with_capacity(self.n_nodes);

                for ii in 0..self.n_nodes {
                    let end_node = &nodes[ii];
                    let expected = end_node.len + 1;

                    let mut len = 0;
                    let mut chain = Vec::with_capacity(expected);
                    let mut next = end_node.pred;

                    while len < expected {
                        if next < ii {
                            let mut sp = result[next].path.clone();
                            if ii < src {
                                sp.reverse();
                            }

                            sp.append(&mut chain);
                            chain = sp;
                            break;
                        }

                        chain.insert(0, next);
                        next = nodes[next].pred;
                        len = chain.len();
                    }

                    result.push(ShortestPath {
                        src: src,
                        dest: ii,
                        dist: end_node.dist,
                        path: chain,
                    });
                }

                result
            }
        };

        result
    }

    /// Finds the shortest paths from a source node to all nodes and returns the intermediate result
    /// for later usage.
    pub fn sssp_dijkstra_lazy(&self, src: usize) -> LazyShortestPaths<W>
    where
        W: Bounded + Num + Zero + PartialOrd + Copy,
    {
        LazyShortestPaths {
            src,
            paths: self.dijkstra(src),
        }
    }

    #[inline]
    fn dijkstra(&self, src: usize) -> Vec<DijNode<W>>
    where
        W: Bounded + Num + Zero + PartialOrd + Copy,
    {
        let mut pq = PairingHeap::<usize, W>::new();
        pq.insert(src, W::zero());

        let mut nodes = vec![DijNode::<W>::new(); self.n_nodes];
        nodes[src].dist = W::zero();
        let mut len = pq.len();

        while len != 0 {
            let (node, prio) = pq.delete_min().unwrap();

            if let Some(nb) = self.neighbours(&node) {
                for (u, dist) in nb {
                    let count = nodes[node].len;
                    let dijnode = &mut nodes[*u];
                    if !dijnode.visited {
                        let alt = prio + *dist;
                        if alt < dijnode.dist {
                            dijnode.dist = alt;
                            dijnode.pred = node;
                            dijnode.len = count + 1;
                            pq.insert(*u, alt);
                        }
                    }
                }
            }

            let dijnode = nodes.get_mut(node).unwrap();
            dijnode.visited = true;
            len = pq.len();
        }

        nodes
    }

    fn insert_weight(&mut self, node1: usize, node2: usize, weight: W) {
        match self.weights.get_mut(&node1) {
            Some(v) => {
                v.push((node2, weight));
            }
            None => {
                let v = vec![(node2, weight)];
                self.weights.insert(node1, v);
            }
        }
    }
}

/// The shortest path from a source node to a destination node.
#[derive(Debug)]
pub struct ShortestPath<W> {
    src: usize,
    dest: usize,
    dist: W,
    path: Vec<usize>,
}

impl<W> ShortestPath<W> {
    /// Returns the index of the source node in the shortest path.
    pub fn src(&self) -> usize {
        self.src
    }

    /// Returns the index of the destination node in the shortest path.
    pub fn dest(&self) -> usize {
        self.dest
    }

    /// Returns the shortest path's distance
    pub fn dist(&self) -> W
    where
        W: Copy,
    {
        self.dist
    }

    /// Returns the path from the source node to destination node.
    ///
    /// The first element of the vector is the source node, the last the destination node.
    pub fn path(&self) -> &Vec<usize> {
        &self.path
    }
}

/// A struct representing the intermediate output of Dijkstra's algorithm.
#[derive(Debug)]
pub struct LazyShortestPaths<W> {
    src: usize,
    paths: Vec<DijNode<W>>,
}

impl<W> LazyShortestPaths<W> {
    /// Returns the shortest path for a given node.
    pub fn get(&self, node_index: usize) -> ShortestPath<W>
    where
        W: Copy,
    {
        let end_node = &self.paths[node_index];
        let expected = end_node.len + 1;

        let mut len = 0;
        let mut chain = Vec::with_capacity(expected);
        chain.push(node_index);
        let mut next = end_node.pred;

        while len < expected {
            chain.insert(0, next);
            next = self.paths[next].pred;
            len = chain.len();
        }

        ShortestPath {
            src: self.src,
            dest: node_index,
            dist: end_node.dist,
            path: chain,
        }
    }

    /// Returns the shortest paths for all nodes.
    pub fn get_all(&self) -> Vec<ShortestPath<W>>
    where
        W: Copy,
    {
        // TODO: remove repeated codes.
        let n_nodes = self.paths.len();
        let mut result: Vec<ShortestPath<W>> = Vec::with_capacity(n_nodes);

        for ii in 0..n_nodes {
            let end_node = &self.paths[ii];
            let expected = end_node.len + 1;

            let mut len = 0;
            let mut chain = Vec::with_capacity(expected);
            let mut next = end_node.pred;

            while len < expected {
                if next < ii {
                    let mut sp = result[next].path.clone();
                    if ii < self.src {
                        sp.reverse();
                    }

                    sp.append(&mut chain);
                    chain = sp;
                    break;
                }

                chain.insert(0, next);
                next = self.paths[next].pred;
                len = chain.len();
            }

            result.push(ShortestPath {
                src: self.src,
                dest: ii,
                dist: end_node.dist,
                path: chain,
            });
        }

        result
    }

    /// Returns the shortest paths for a given list of node indices.
    pub fn get_list(&self, node_indices: &[usize]) -> Vec<ShortestPath<W>>
    where
        W: Copy,
    {
        let mut result = Vec::with_capacity(node_indices.len());

        for ii in node_indices {
            let end_node = &self.paths[*ii];
            let expected = end_node.len + 1;

            let mut len = 0;
            let mut chain = Vec::with_capacity(expected);
            chain.push(*ii);
            let mut next = end_node.pred;

            while len < expected {
                chain.insert(0, next);
                next = self.paths[next].pred;
                len = chain.len();
            }

            result.push(ShortestPath {
                src: self.src,
                dest: *ii,
                dist: end_node.dist,
                path: chain,
            });
        }

        result
    }
}

#[derive(Clone, Debug)]
struct DijNode<W> {
    /// Id of the predecessor's node in SSSP solution from Dijkstra's algorithm.
    pred: usize,
    /// Distance to the predecessor.
    dist: W,
    /// Flag whether a node is visisted or not.
    visited: bool,
    len: usize,
}

impl<W> DijNode<W> {
    pub fn new() -> Self
    where
        W: Bounded,
    {
        Self {
            pred: 0,
            dist: <W as Bounded>::max_value(),
            visited: false,
            len: 0,
        }
    }
}
