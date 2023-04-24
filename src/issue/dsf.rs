//! Sometimes, we need to keep track of k disjoint groups of
//! items — meaning that each each item uniquely belongs to one group.
//! The most common operations that we'd like to run on our collection of groups are:
//! find_set(x) which tells us which of our k groups u belongs to and union(s1, s2)
//! which allows us to merge two groups. Remember in the section on connected
//! components we needed to quickly find out if a node was already part of some component.

#![allow(dead_code)]

use std::marker::PhantomData;

/// A single node in the disjoit set forest
#[derive(Debug, Eq, PartialEq)]
pub struct DSFNode<T: PartialEq> {
    /// The payload stored at this node. This is a unique
    /// identifier of a node
    key: T,

    /// Each node must have a parent. The root node
    /// is its own parent
    parent: usize,

    /// The location of this node in forest
    index: usize,

    /// With each node, we maintain an integer value x.rank
    /// This is an upper bound on the height of x -- that is
    /// an upper bound on the number of edges in the longest
    /// x -> descendant_leaf simple path. make_set, initializes
    /// this value to 0
    rank: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DSFNodeHandle<T>(usize, PhantomData<T>);

impl<T: PartialEq> DSFNode<T> {
    /// Create a new node with the given value and parent.
    /// On creation, the parent field should point to
    /// the location of this node in the forest vector --
    /// that is, a singleton node is its own parent
    pub fn new(key: T, parent: usize) -> Self {
        let rank = 0;
        let index = parent;
        DSFNode {
            key,
            parent,
            rank,
            index,
        }
    }
}
/// A disjoint set forest is a collection of trees.
/// Only the nodes in a single tree are linked together.
/// A user interacts with the forest using the nodes
#[derive(Debug)]
pub struct DSF<T: PartialEq> {
    /// A collection of all the nodes in the tree
    forest: Vec<DSFNode<T>>,
}

impl<T: PartialEq> DSF<T> {
    /// Creates a new disjoint set forest structure with no trees in it
    pub fn new() -> Self {
        DSF { forest: Vec::new() }
    }

    /// Adds a new node into the disjoint set forest.
    /// It returns a handle to a node that can be passed into
    /// the other two methods
    pub fn make_set(&mut self, x: T) -> DSFNodeHandle<T> {
        let idx = self.forest.len();
        self.forest.push(DSFNode::new(x, idx));
        DSFNodeHandle(idx, PhantomData)
    }

    /// Th union operation has two bcases. if the roots have unequal rank, we make
    /// the root with lower rank point to the root with the higher rank. The
    /// ranks, however, do not change. If the roots have equal ranke, we choose one
    /// of the roots as the root of the combined set. We also increase
    /// the rank of the new root by 1.
    pub fn union(&mut self, a: &DSFNodeHandle<T>, b: &DSFNodeHandle<T>) {
        let a_root = Self::find_set_helper(&mut self.forest, a.0);
        let b_root = Self::find_set_helper(&mut self.forest, b.0);

        // We make the root with the higher rank the parent of the one
        // with the lower rank. This effectively makes it the representative
        // of the combined set
        if self.forest[a_root].rank > self.forest[b_root].rank {
            self.forest[b_root].parent = self.forest[a_root].index
        } else {
            self.forest[a_root].parent = self.forest[b_root].index;

            // Note that ranks only change when we merge two trees with the same
            // ranks. The choice of whose rank to increase is made arbitrarily
            if self.forest[a_root].rank == self.forest[b_root].rank {
                self.forest[b_root].rank += 1;
            }
        }
    }

    /// Finds the representative if x. Also does path compression. It does not change
    /// the value of rank.
    pub fn find_set(&mut self, x: &DSFNodeHandle<T>) -> DSFNodeHandle<T> {
        let idx = Self::find_set_helper(&mut self.forest, x.0);
        DSFNodeHandle(idx, PhantomData)
    }

    fn find_set_helper(forest: &mut Vec<DSFNode<T>>, x_index: usize) -> usize {
        // When I first saw this method, I simply thought it was the coolest
        // thing in the world. Recursion on recursive structures yields
        // simple, elegant, yet powerful code. According to CLRS, this
        // is an instance of a general method called `the two-pass` method.

        // First make an upward pass to find the representative, i.e the root
        // then make a downward pass, as the stack is being unwound, to set
        // the parent of each node in the x -> root path
        let cur_x_parent = forest[x_index].parent;
        if cur_x_parent != x_index {
            forest[x_index].parent = Self::find_set_helper(forest, cur_x_parent);
        }
        forest[x_index].parent
    }
}
