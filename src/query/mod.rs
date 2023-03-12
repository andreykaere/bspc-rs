use crate::selectors::Selector;
use crate::BspwmConnection;

pub mod tree;

use tree::Tree;

pub enum QueryOptions {
    Monitor,
    Desktop,
    Node,
}

impl BspwmConnection {
    pub fn query_nodes(&mut self) -> Vec<i32> {
        todo!();
    }

    pub fn query_desktops(&mut self) -> Vec<i32> {
        todo!();
    }

    pub fn query_monitors(&mut self) -> Vec<i32> {
        todo!();
    }

    pub fn query_tree(
        &mut self,
        options: QueryOptions,
        selector: Selector,
    ) -> Tree {
        todo!();
    }
}
