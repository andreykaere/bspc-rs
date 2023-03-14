use std::string::ToString;
use strum_macros::Display;

use crate::communication::BspcCommunication;
use crate::errors::ReplyError;
use crate::parser::utils::from_hex_to_id;
use crate::selectors::Selector;
use crate::tree::{Node, Tree};
use crate::{BspwmConnection, Id};

#[derive(Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum QueryOptions {
    Monitor,
    Desktop,
    Node,
}

impl BspwmConnection {
    fn query(
        query_type: &str,
        names_flag: bool,
        selector: Option<&str>,
        monitor_selector: Option<&str>,
        desktop_selector: Option<&str>,
        node_selector: Option<&str>,
    ) -> Result<Vec<Id>, ReplyError> {
        let mut conn = BspwmConnection::connect()?;
        let mut request = format!("--{query_type}");

        if names_flag {
            if query_type == "nodes" {
                return Err(ReplyError::InvalidRequest);
            } else {
                request = format!("{request}\x00--names");
            }
        }

        if let Some(sel) = selector {
            request = format!("{request}\x00{sel}");
        }

        if let Some(sel) = monitor_selector {
            request = format!("{request}\x00--monitor\x00{sel}");
        }

        if let Some(sel) = desktop_selector {
            request = format!("{request}\x00--desktop\x00{sel}");
        }

        if let Some(sel) = node_selector {
            request = format!("{request}\x00--node\x00{sel}");
        }

        let message = format!("query\x00{}\x00", request);
        conn.send_message(&message)?;

        let reply = conn.receive_message()?;
        let mut ids = Vec::new();

        for node_id in reply.split('\n') {
            if !node_id.is_empty() {
                let id = from_hex_to_id(node_id)?;
                ids.push(id);
            }
        }

        Ok(ids)
    }

    pub fn query_nodes(
        selector: Option<&str>,
        monitor_selector: Option<&str>,
        desktop_selector: Option<&str>,
        node_selector: Option<&str>,
    ) -> Result<Vec<Id>, ReplyError> {
        Self::query(
            "nodes",
            false,
            selector,
            monitor_selector,
            desktop_selector,
            node_selector,
        )
    }

    pub fn query_desktops(
        names_flag: bool,
        selector: Option<&str>,
        monitor_selector: Option<&str>,
        desktop_selector: Option<&str>,
        node_selector: Option<&str>,
    ) -> Result<Vec<Id>, ReplyError> {
        Self::query(
            "desktops",
            names_flag,
            selector,
            monitor_selector,
            desktop_selector,
            node_selector,
        )
    }

    pub fn query_monitors(
        names_flag: bool,
        selector: Option<&str>,
        monitor_selector: Option<&str>,
        desktop_selector: Option<&str>,
        node_selector: Option<&str>,
    ) -> Result<Vec<Id>, ReplyError> {
        Self::query(
            "monitors",
            names_flag,
            selector,
            monitor_selector,
            desktop_selector,
            node_selector,
        )
    }

    pub fn query_tree(option: QueryOptions) -> Result<Tree, ReplyError> {
        let mut conn = BspwmConnection::connect()?;
        let message =
            format!("query\x00--tree\x00--{}\x00", option.to_string());
        conn.send_message(&message)?;

        let reply = conn.receive_message()?;

        match option {
            QueryOptions::Monitor => {
                Ok(Tree::Monitor(serde_json::from_str(&reply)?))
            }

            QueryOptions::Desktop => {
                Ok(Tree::Desktop(serde_json::from_str(&reply)?))
            }

            QueryOptions::Node => Ok(Tree::Node(serde_json::from_str(&reply)?)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::events::*;
    // use std::error::Error;
    // use std::io::{self, Read, Write};
    // use std::os::unix::net::UnixStream;

    #[test]
    fn test_query_nodes() {
        println!(
            "{:#?}",
            BspwmConnection::query_nodes(None, None, None, Some(".!hidden"))
                .unwrap()
        );
    }

    #[test]
    fn test_query_tree() {
        let tree = BspwmConnection::query_tree(QueryOptions::Monitor).unwrap();

        println!("{tree:#?}");
    }
}
