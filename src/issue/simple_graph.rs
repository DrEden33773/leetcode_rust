use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}
#[allow(dead_code)]
impl State {
    pub fn new(cost: usize, position: usize) -> Self {
        State { cost, position }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.position.cmp(&self.position))
        // another version could be:
        //
        // other
        //     .cost
        //     .cmp(&self.cost)
        //     .then_with(|| self.position.cmp(&other.position))
    }
}
// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[allow(dead_code)]
pub struct Edge {
    node: usize,
    cost: usize,
}
#[allow(dead_code)]
impl Edge {
    pub fn new(node: usize, cost: usize) -> Self {
        Edge { node, cost }
    }
}

/// ## Dijkstra's shortest path algorithm.
///
/// Start at `start` and use `dist` to track the current shortest distance
/// to each node.
///
/// This implementation isn't memory-efficient as it may leave duplicate
/// nodes in the queue.
///
/// It also uses `usize::MAX` as a sentinel value,
/// for a simpler implementation.
pub fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = vec![usize::MAX; adj_list.len()];

    // heap => only contains those who could arrive to `start`
    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State::new(0, start));

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost: start_to_picked,
        position: picked_node,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        if picked_node == goal {
            return Some(start_to_picked);
        }
        // Important as we may have already found a better way,
        // in this case we could continue to save time
        if start_to_picked > dist[picked_node] {
            continue;
        }
        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for Edge {
            cost: picked_to_curr,
            node: curr_node,
        } in &adj_list[picked_node]
        {
            let new_curr = State {
                cost: start_to_picked + *picked_to_curr,
                position: *curr_node,
            };
            // If so, add it to the frontier and continue
            if new_curr.cost < dist[*curr_node] {
                heap.push(new_curr);
                // We have found a better way, update dist
                dist[*curr_node] = new_curr.cost;
            }
        }
    }

    // Goal not reachable
    None
}

#[cfg(test)]
mod dijkstra {
    use super::*;

    /// This is the directed graph we're going to use.
    ///
    /// The node numbers correspond to the different states,
    /// and the edge weights symbolize the cost of moving
    /// from one node to another.
    ///
    /// Note that the edges are one-way.
    ///
    /// ```txt
    ///                  7
    ///          +-----------------+
    ///          |                 |
    ///          v   1        2    |  2
    ///          0 -----> 1 -----> 3 ---> 4
    ///          |        ^        ^      ^
    ///          |        | 1      |      |
    ///          |        |        | 3    | 1
    ///          +------> 2 -------+      |
    ///           10      |               |
    ///                   +---------------+
    /// ```
    ///
    /// The graph is represented as an adjacency list where each index,
    /// corresponding to a node value, has a list of outgoing edges.
    ///
    /// Chosen for its efficiency.
    #[test]
    fn it_works() {
        let graph = vec![
            // Node 0
            vec![Edge::new(2, 10), Edge::new(1, 1)],
            // Node 1
            vec![Edge::new(3, 2)],
            // Node 2
            vec![Edge::new(1, 1), Edge::new(3, 3), Edge::new(4, 1)],
            // Node 3
            vec![Edge::new(0, 7), Edge::new(4, 2)],
            // Node 4
            vec![],
        ];
        assert_eq!(shortest_path(&graph, 0, 1), Some(1));
        assert_eq!(shortest_path(&graph, 0, 3), Some(3));
        assert_eq!(shortest_path(&graph, 3, 0), Some(7));
        assert_eq!(shortest_path(&graph, 0, 4), Some(5));
        assert_eq!(shortest_path(&graph, 4, 0), None);
    }
}
