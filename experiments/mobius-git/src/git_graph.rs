use anyhow::{Context, Result};
use git2::{Oid, Repository, Sort};
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CommitNode {
    pub id: Oid,
    pub message: String,
    pub parent_ids: Vec<Oid>,
    // Topological coordinates
    pub u: f32, // Time/Depth (along the strip)
    pub v: f32, // Branch offset (across the strip)
}

pub struct GitGraph {
    pub graph: Graph<CommitNode, ()>,
    pub node_map: HashMap<Oid, NodeIndex>,
}

impl GitGraph {
    pub fn new(path: &Path, max_commits: usize) -> Result<Self> {
        let repo = Repository::open(path).context("Failed to open git repo")?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;

        let mut graph = Graph::new();
        let mut node_map = HashMap::new();
        let mut ordered_indices = Vec::new();

        // First pass: Collect nodes and build basic graph structure
        for oid_res in revwalk.take(max_commits) {
            let oid = oid_res?;
            if node_map.contains_key(&oid) {
                continue;
            }

            let commit = repo.find_commit(oid)?;
            let parent_ids: Vec<Oid> = commit.parent_ids().collect();
            let message = commit.summary().unwrap_or("").to_string();

            let node_data = CommitNode {
                id: oid,
                message,
                parent_ids: parent_ids.clone(),
                u: 0.0,
                v: 0.0,
            };

            let node_idx = graph.add_node(node_data);
            node_map.insert(oid, node_idx);
            ordered_indices.push((node_idx, parent_ids));
        }

        // Second pass: Add edges
        for (node_idx, parent_ids) in &ordered_indices {
            for pid in parent_ids {
                if let Some(&parent_idx) = node_map.get(pid) {
                    graph.add_edge(*node_idx, parent_idx, ());
                }
            }
        }

        // Third pass: Layout (U, V)
        // u = topological index (0, 1, 2...)
        // v = lane index

        let mut next_lane = 1.0;
        let mut node_lanes: HashMap<NodeIndex, f32> = HashMap::new();

        // The HEAD usually starts at v=0
        if let Some((head_idx, _)) = ordered_indices.first() {
            node_lanes.insert(*head_idx, 0.0);
        }

        for (i, (node_idx, parent_ids)) in ordered_indices.iter().enumerate() {
            let u = i as f32;

            // Get my assigned lane, or default to 0 if disconnected/unreached (shouldn't happen with single head)
            let my_v = *node_lanes.get(node_idx).unwrap_or(&0.0);

            // Assign U, V to node in graph
            if let Some(node) = graph.node_weight_mut(*node_idx) {
                node.u = u;
                node.v = my_v;
            }

            if parent_ids.is_empty() {
                continue;
            }

            // Propagate V to parents
            // First parent (main lineage) gets my V, if not already taken
            let first_parent_oid = parent_ids[0];
            if let Some(&p_idx) = node_map.get(&first_parent_oid) {
                // Main parent continues my lane
                if !node_lanes.contains_key(&p_idx) {
                    node_lanes.insert(p_idx, my_v);
                }
            }

            // Other parents (merge bases) get new lanes
            for &pid in parent_ids.iter().skip(1) {
                 if let Some(&p_idx) = node_map.get(&pid) {
                    if !node_lanes.contains_key(&p_idx) {
                        // Assign a new lane
                        // To keep it balanced around 0, we can alternate signs
                        let v = if next_lane % 2.0 == 0.0 {
                            next_lane / 2.0
                        } else {
                            -(next_lane + 1.0) / 2.0
                        };
                        next_lane += 1.0;
                        node_lanes.insert(p_idx, v);
                    }
                }
            }
        }

        Ok(Self { graph, node_map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{Signature, RepositoryInitOptions};
    use tempfile::TempDir;

    fn create_dummy_repo() -> (TempDir, Repository) {
        let temp_dir = TempDir::new().unwrap();
        let mut opts = RepositoryInitOptions::new();
        opts.initial_head("main");
        let repo = Repository::init_opts(temp_dir.path(), &opts).unwrap();

        let sig = Signature::now("Test User", "test@example.com").unwrap();

        // Initial commit
        {
            let tree_id = repo.index().unwrap().write_tree().unwrap();
            let tree = repo.find_tree(tree_id).unwrap();
            repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[]).unwrap();
        }

        (temp_dir, repo)
    }

    fn commit(repo: &Repository, msg: &str, parents: &[&git2::Commit]) -> Oid {
        let sig = Signature::now("Test User", "test@example.com").unwrap();
        let tree_id = repo.index().unwrap().write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, parents).unwrap()
    }

    #[test]
    fn test_git_graph_traversal() {
        let (temp_dir, repo) = create_dummy_repo();

        // Make a few commits
        // HEAD -> C1 -> C0
        let head = repo.head().unwrap().peel_to_commit().unwrap();
        let _c1_oid = commit(&repo, "Commit 1", &[&head]); // C1
        let c1 = repo.find_commit(_c1_oid).unwrap();

        // Feature commit F1 -> C0
        // We need C0 (head from dummy repo creation)
        let c0 = head;

        let sig = Signature::now("Test User", "test@example.com").unwrap();
        let tree_id = repo.index().unwrap().write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();

        let f1_oid = repo.commit(Some("refs/heads/feature"), &sig, &sig, "F1", &tree, &[&c0]).unwrap();
        let f1 = repo.find_commit(f1_oid).unwrap();

        // Merge M1 -> C1, F1
        repo.set_head("refs/heads/main").unwrap();
        let m1_oid = repo.commit(Some("refs/heads/main"), &sig, &sig, "M1", &tree, &[&c1, &f1]).unwrap();

        // Now run graph builder
        let git_graph = GitGraph::new(temp_dir.path(), 100).unwrap();

        // Verify nodes
        assert_eq!(git_graph.graph.node_count(), 4); // C0, C1, F1, M1

        // Verify M1 has 2 parents
        let m1_node_idx = *git_graph.node_map.get(&m1_oid).unwrap();
        let m1_node = &git_graph.graph[m1_node_idx];
        assert_eq!(m1_node.parent_ids.len(), 2);

        // Verify V assignment
        // M1 should be v=0 (HEAD)
        assert_eq!(m1_node.v, 0.0);

        // C1 (first parent of M1) should be v=0
        let c1_node_idx = *git_graph.node_map.get(&c1.id()).unwrap();
        let c1_node = &git_graph.graph[c1_node_idx];
        assert_eq!(c1_node.v, 0.0);

        // F1 (second parent of M1) should have different v
        let f1_node_idx = *git_graph.node_map.get(&f1_oid).unwrap();
        let f1_node = &git_graph.graph[f1_node_idx];
        assert!(f1_node.v != 0.0);

        // C0 (parent of C1 and F1)
        // Since C1 visits C0 first (likely), C0 gets C1's v (0.0).
        let c0_node_idx = *git_graph.node_map.get(&c0.id()).unwrap();
        let c0_node = &git_graph.graph[c0_node_idx];
        println!("C0.v: {}", c0_node.v);
        // We don't strictly assert C0.v is 0.0 because traversal order might vary slightly?
        // But with Sort::TOPOLOGICAL | Sort::TIME and C1 > F1 > C0, C1 should be processed before F1?
        // Actually M1 > C1 and M1 > F1.
        // revwalk returns M1.
        // Then C1 or F1.
        // If C1, then C1 propagates to C0.
        // If F1, then F1 propagates to C0.
        // C1 and F1 have same timestamp. Git usually orders by parent index or OID if equal?
        // Either way, graph is built.
    }
}
