pub mod array;
pub mod back_trace;
pub mod binary_search;
pub mod binary_tree;
pub mod bit_manipulation;
pub mod data_stream;
pub mod disjoint_set;
pub mod dp;
pub mod graph;
pub mod greedy;
pub mod hash_table;
pub mod heap;
pub mod mathematical;
pub mod prefix_sum;
pub mod recursion;
pub mod simulation;
pub mod sliding_window;
pub mod sort;
pub mod string;

/// # Solution!
///
/// ## Macro
///
/// ### Replace:
///
/// ```
/// pub struct Solution;
/// ```
#[macro_export] /* => export to `crate` */
macro_rules! Solution {
    () => {
        pub struct Solution;
    };
}

/// # sln!
///
/// ## Macro
///
/// ### Replace:
///
/// ```
/// pub struct Solution;
/// ```
#[macro_export] /* => export to `crate` */
macro_rules! sln {
    () => {
        pub struct Solution;
    };
}
