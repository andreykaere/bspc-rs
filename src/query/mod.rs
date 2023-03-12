use crate::BspwmConnection;

pub mod tree;

use tree::Tree;

pub enum QueryOptions {
    Monitor,
    Desktop,
    Node,
}

pub struct MonitorSelector {}
pub struct DesktopSelector {}
pub struct NodeSelector {}

pub enum QuerySelectors {
    Monitor(MonitorSelector),
    Desktop(DesktopSelector),
    Node(NodeSelector),
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
        opts: QueryOptions,
        sel: QuerySelectors,
    ) -> Tree {
        todo!();
    }
}
