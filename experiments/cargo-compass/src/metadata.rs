use anyhow::Result;
use cargo_metadata::{MetadataCommand, Package};

pub fn fetch_packages() -> Result<Vec<Package>> {
    let metadata = MetadataCommand::new().exec()?;
    let workspace_members = metadata.workspace_members;

    // Filter packages that are in the workspace members
    // We sort them by name for consistent UI
    let mut packages: Vec<Package> = metadata
        .packages
        .into_iter()
        .filter(|p| workspace_members.contains(&p.id))
        .collect();

    packages.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(packages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_packages() {
        let packages = fetch_packages().expect("Failed to fetch packages");
        assert!(!packages.is_empty());

        // Ensure we find ourselves
        let compass = packages.iter().find(|p| p.name == "cargo-compass");
        assert!(
            compass.is_some(),
            "Could not find cargo-compass in workspace"
        );
    }
}
