pub mod desktop_events;
pub mod errors;
pub mod monitor_events;
pub mod node_events;
pub mod parse_common;
mod utils;

use super::tree::Tree;
use errors::ParseError;

use std::str::FromStr;

impl FromStr for Tree {
    type Err = serde_json::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tree: Tree = serde_json::from_str(input)?;

        Ok(tree)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::process::Command;

    #[test]
    fn parse_tree() {
        let tree = Command::new("bspc")
            .arg("query")
            .arg("-T")
            .arg("-m")
            .output()
            .unwrap();

        let tree = String::from_utf8_lossy(&tree.stdout);

        println!("{:#?}", tree.parse::<Tree>().unwrap());
    }
}
