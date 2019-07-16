// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Lazy grouping by key across boundaries.

use std::hash::Hash;
use std::collections::HashSet;

pub struct Group<'a, K: Eq, E> {
    pub key: K,
    key_fn: &'a Fn(&E)->K,
    pub iter: Box<Iterator<Item=&'a E> + 'a>,
}

impl<'a, K: Eq, E> Iterator for Group<'a, K, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<&'a E> {
        while let Some(e) = self.iter.next() {
            if (self.key_fn)(&e) == self.key {
                return Some(e)
            }
        }
        None
    }
}

pub struct GroupBy<'a, K: Eq + Hash, E> {
    elems: &'a [E],
    key_fn: &'a Fn(&E)->K,
    seen: HashSet<K>,
}

impl<'a, K: Eq + Hash + Clone, E> Iterator for GroupBy<'a, K, E> {
    type Item = Group<'a, K, E>;

    fn next(&mut self) -> Option<Group<'a, K, E>> {
        while !self.elems.is_empty() {
            let k = (self.key_fn)(&self.elems[0]);
            if !self.seen.contains(&k) {
                self.seen.insert(k.clone());
                let result = Group {
                    key: k,
                    key_fn: self.key_fn,
                    iter: Box::new(self.elems.iter()),
                };
                self.elems = &self.elems[1..];
                return Some(result);
            }
            self.elems = &self.elems[1..];
        }
        None
    }
}

pub trait GroupByExt<'a, K, E, F>
    where K: Eq + Hash, F: Fn(&E)->K
{
    fn group_by(&'a self, key_fn: &'a F) -> GroupBy<'a, K, E>;
}

impl<'a, K, E, F> GroupByExt<'a, K, E, F> for [E]
    where K: Eq + Hash, F: Fn(&E)->K
{
    
    fn group_by(&'a self, key_fn: &'a F) -> GroupBy<'a, K, E> {
        GroupBy{ elems: self, key_fn, seen: HashSet::new() }
    }
}
