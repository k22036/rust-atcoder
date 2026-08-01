#![allow(unused_imports)]
#![allow(non_snake_case)]

use ac_library::dsu::Dsu;
use ac_library::fenwicktree::FenwickTree;
use ac_library::scc::SccGraph;
use ac_library::{inv_mod, pow_mod};
use itertools::{ArrayCombinations, Tuples, iproduct};
use itertools::{Itertools, Permutations, all, any, enumerate};
use proconio::input;
use proconio::marker::Bytes;
use rand::{random, random_range};
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::hash_map::Entry::Occupied;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::fmt::format;
use std::fs::set_permissions;
use std::hash::Hash;
use std::iter::{Map, once, zip};
use std::mem::swap;
use std::ops::Bound::{Included, Unbounded};
use std::ops::{Index, Range};
use std::{isize, usize};

fn main() {
    input! {
        n: usize,
    }
}
