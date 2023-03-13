use crate::selectors::Selector;
use crate::BspwmConnection;

use crate::tree::Node;
use crate::tree::Tree;

pub enum QueryOptions {
    Monitor,
    Desktop,
    Node,
}

impl BspwmConnection {
    pub fn query_nodes(&mut self, selector: &str) -> Vec<Node> {
        todo!();
    }

    pub fn query_desktops(&mut self, selector: &str) -> Vec<Node> {
        todo!();
    }

    pub fn query_monitors(&mut self, selector: &str) -> Vec<Node> {
        todo!();
    }

    pub fn query_tree(
        &mut self,
        options: QueryOptions,
        // selector: Selector,
        selector: &str,
    ) -> Tree {
        todo!();
    }
}
