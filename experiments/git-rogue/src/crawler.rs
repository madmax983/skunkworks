use anyhow::Result;
use git_associates::GitModel;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub hash: String,
    pub short_hash: String,
    pub message: String,
    pub author: String,
    pub parents: Vec<String>,
    pub children: Vec<String>,
}

pub fn crawl(path: &str, limit: usize) -> Result<Vec<Node>> {
    let model = GitModel::open(path)?;
    let commits = model.history(limit)?;

    let mut nodes = Vec::new();
    // Use an index map to quickly find node by hash in the vector
    let mut node_indices: HashMap<String, usize> = HashMap::new();

    for commit in commits {
        if node_indices.contains_key(&commit.hash) {
            continue;
        }

        let node = Node {
            hash: commit.hash.clone(),
            short_hash: commit.short_hash.clone(),
            message: commit.message.clone(),
            author: commit.author.clone(),
            parents: commit.parents.clone(),
            children: Vec::new(),
        };

        node_indices.insert(commit.hash.clone(), nodes.len());
        nodes.push(node);
    }

    // Post-process to populate children
    let mut parent_to_children: HashMap<String, Vec<String>> = HashMap::new();

    for node in &nodes {
        for parent_hash in &node.parents {
            parent_to_children
                .entry(parent_hash.clone())
                .or_default()
                .push(node.hash.clone());
        }
    }

    // Apply children
    for node in &mut nodes {
        if let Some(children) = parent_to_children.get(&node.hash) {
            node.children = children.clone();
        }
    }

    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crawl_current_repo() {
        // We assume we are in a git repo
        let nodes = crawl(".", 10).expect("Failed to crawl repo");
        assert!(!nodes.is_empty());
        let head = &nodes[0];
        println!("HEAD: {} - {}", head.short_hash, head.message);

        // HEAD should have parents (unless it's the very first commit, unlikely here)
        // HEAD should not have children in this localized graph traversal (since we started at HEAD)
        // Unless we crawled multiple branches, but revwalk starting at HEAD only sees ancestors.
        // So HEAD.children should be empty.
        assert!(head.children.is_empty());

        if nodes.len() > 1 {
            // The second node is likely a parent of the first
            let parent = &nodes[1];
            // So the parent should have the first node as a child
            assert!(parent.children.contains(&head.hash));
        }
    }
}
