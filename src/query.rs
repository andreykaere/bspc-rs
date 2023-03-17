use std::string::ToString;
use strum_macros::Display;

use crate::communication::BspcCommunication;
use crate::errors::{QueryError, ReplyError};
use crate::parser::utils::from_hex_to_id;
use crate::selectors::{
    DesktopSelector, MonitorSelector, NodeSelector, Selector,
};
use crate::tree::{Node, Tree};
use crate::{bspc, Id};

#[derive(Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum QueryOptions {
    Monitor,
    Desktop,
    Node,
}

fn query(
    query_type: &str,
    names_flag: bool,
    selector: Option<&str>,
    monitor_selector: Option<&str>,
    desktop_selector: Option<&str>,
    node_selector: Option<&str>,
) -> Result<Vec<Id>, ReplyError> {
    let mut conn = bspc::connect()?;
    let mut request = format!("query\x00--{query_type}\x00");

    if names_flag {
        if query_type == "nodes" {
            return Err(ReplyError::QueryError(QueryError::InvalidRequest(
                "You can't apply --names for nodes query request".to_string(),
            )));
        } else {
            request = format!("{request}--names\x00");
        }
    }

    if let Some(sel) = selector {
        request = format!("{request}{sel}\x00");
    }

    if let Some(sel) = monitor_selector {
        request = format!("{request}--monitor\x00{sel}\x00");
    }

    if let Some(sel) = desktop_selector {
        request = format!("{request}--desktop\x00{sel}\x00");
    }

    if let Some(sel) = node_selector {
        request = format!("{request}--node\x00{sel}\x00");
    }

    conn.send_message(&request)?;

    let reply = conn.receive_message()?;

    let mut ids = Vec::new();

    for reply_id in reply.iter() {
        let id = from_hex_to_id(reply_id)?;
        ids.push(id);
    }

    Ok(ids)
}

pub fn query_nodes(
    selector: Option<NodeSelector>,
    monitor_selector: Option<MonitorSelector>,
    desktop_selector: Option<DesktopSelector>,
    node_selector: Option<NodeSelector>,
) -> Result<Vec<Id>, ReplyError> {
    query(
        "nodes",
        false,
        extract(&selector)?,
        extract(&monitor_selector)?,
        extract(&desktop_selector)?,
        extract(&node_selector)?,
    )
}

pub fn query_desktops(
    names_flag: bool,
    selector: Option<DesktopSelector>,
    monitor_selector: Option<MonitorSelector>,
    desktop_selector: Option<DesktopSelector>,
    node_selector: Option<NodeSelector>,
) -> Result<Vec<Id>, ReplyError> {
    query(
        "desktops",
        names_flag,
        extract(&selector)?,
        extract(&monitor_selector)?,
        extract(&desktop_selector)?,
        extract(&node_selector)?,
    )
}

fn extract<S>(selector: &Option<S>) -> Result<Option<&str>, ReplyError>
where
    S: Selector,
{
    if let Some(sel) = selector.as_ref() {
        if !sel.is_valid() {
            return Err(ReplyError::InvalidSelector(format!(
                "This {} selector is invalid: '{}'",
                sel.kind(),
                sel.extract(),
            )));
        }

        return Ok(Some(sel.extract()));
    }

    Ok(None)
}
pub fn query_monitors(
    names_flag: bool,
    selector: Option<MonitorSelector>,
    monitor_selector: Option<MonitorSelector>,
    desktop_selector: Option<DesktopSelector>,
    node_selector: Option<NodeSelector>,
) -> Result<Vec<Id>, ReplyError> {
    query(
        "monitors",
        names_flag,
        extract(&selector)?,
        extract(&monitor_selector)?,
        extract(&desktop_selector)?,
        extract(&node_selector)?,
    )
}

/// Returnes tree representation of the matching item
/// Note: when more then one of the arguments are not `None`, then the
/// matching will give the result in this priority: Node, Desktop, Monitor.
/// For example, if Desktop and Node are both not `None`, than this will give the
/// output for Node.
pub fn query_tree(
    monitor_selector: Option<MonitorSelector>,
    desktop_selector: Option<DesktopSelector>,
    node_selector: Option<NodeSelector>,
) -> Result<Tree, ReplyError> {
    let mut conn = bspc::connect()?;
    let mut request = "query\x00--tree\x00".to_string();

    let monitor_selector = extract(&monitor_selector)?;
    let desktop_selector = extract(&desktop_selector)?;
    let node_selector = extract(&node_selector)?;

    if let Some(sel) = monitor_selector {
        request = format!("{request}--monitor\x00{sel}\x00");
    }

    if let Some(sel) = desktop_selector {
        request = format!("{request}--desktop\x00{sel}\x00");
    }

    if let Some(sel) = node_selector {
        request = format!("{request}--node\x00{sel}\x00");
    }

    conn.send_message(&request)?;

    let reply = conn.receive_message()?;

    if reply.len() > 1 {
        // TODO: Test if this can happen
        panic!("{}", format!("Something is weird, reply has more than one element, this is debug log: {:#?}", reply));
    }

    let reply = &reply[0];

    if node_selector.is_some() {
        return Ok(Tree::Node(serde_json::from_str(reply)?));
    }

    if desktop_selector.is_some() {
        return Ok(Tree::Desktop(serde_json::from_str(reply)?));
    }

    if monitor_selector.is_some() {
        return Ok(Tree::Monitor(serde_json::from_str(reply)?));
    }

    Err(ReplyError::QueryError(QueryError::InvalidRequest(
        "No options were given".to_string(),
    )))

    // match option {
    //     QueryOptions::Monitor => {
    //         Ok(Tree::Monitor(serde_json::from_str(reply)?))
    //     }

    //     QueryOptions::Desktop => {
    //         Ok(Tree::Desktop(serde_json::from_str(reply)?))
    //     }

    //     QueryOptions::Node => Ok(Tree::Node(serde_json::from_str(reply)?)),
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::events::*;
    // use std::error::Error;
    // use std::io::{self, Read, Write};
    // use std::os::unix::net::UnixStream;

    // #[test]
    // fn test_query_nodes() {
    //     println!(
    //         "{:#?}",
    //         Bspc::query_nodes(None, None, None, Some(".!hidden"))
    //             .unwrap()
    //     );
    // }

    #[test]
    fn test_fullscreen_node() {
        let node_request = format!(".fullscreen.window");
        let query_result =
            bspc::query_nodes(None, None, None, Some(&node_request));

        query_result.unwrap();
    }

    #[test]
    fn test_query_tree() {
        let tree = bspc::query_tree(QueryOptions::Monitor).unwrap();

        println!("{tree:#?}");
    }
}
