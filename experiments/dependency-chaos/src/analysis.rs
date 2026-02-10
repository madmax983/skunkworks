use anyhow::Result;
use cargo_metadata::{MetadataCommand, DependencyKind};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct CrateAnalysis {
    pub name: String,
    pub mass1: f32, // Source bytes
    pub mass2: f32, // Test bytes
    pub len1: f32,  // Deps count
    pub len2: f32,  // Dev-deps count
    pub color: (u8, u8, u8),
}

pub fn analyze_workspace() -> Result<Vec<CrateAnalysis>> {
    let metadata = MetadataCommand::new().exec()?;
    let workspace_members = &metadata.workspace_members;

    let mut results = Vec::new();

    for pkg_id in workspace_members {
        if let Some(pkg) = metadata.packages.iter().find(|p| &p.id == pkg_id) {
            let root = pkg.manifest_path.parent().unwrap();
            let src_path = root.join("src");
            let tests_path = root.join("tests");
            let examples_path = root.join("examples");

            let mass1 = count_bytes(src_path.as_std_path());
            let mass2 = count_bytes(tests_path.as_std_path()) + count_bytes(examples_path.as_std_path());

            let len1 = pkg.dependencies.iter().filter(|d| d.kind == DependencyKind::Normal).count() as f32;
            let len2 = pkg.dependencies.iter().filter(|d| d.kind == DependencyKind::Development).count() as f32;

            let color = generate_color(&pkg.name);

            results.push(CrateAnalysis {
                name: pkg.name.clone(),
                mass1: mass1 as f32,
                mass2: mass2 as f32,
                len1,
                len2,
                color,
            });
        }
    }

    Ok(results)
}

fn count_bytes(path: &Path) -> u64 {
    if !path.exists() { return 0; }
    let mut count = 0;
    if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                count += count_bytes(&entry.path());
            }
        }
    } else {
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                if let Ok(metadata) = std::fs::metadata(path) {
                    count = metadata.len();
                }
            }
        }
    }
    count
}

fn generate_color(name: &str) -> (u8, u8, u8) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let hash = hasher.finish();

    let h = (hash % 360) as f32 / 360.0;
    let s = 0.8;
    let l = 0.5;

    hsl_to_rgb(h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0/6.0 {
        (c, x, 0.0)
    } else if h < 2.0/6.0 {
        (x, c, 0.0)
    } else if h < 3.0/6.0 {
        (0.0, c, x)
    } else if h < 4.0/6.0 {
        (0.0, x, c)
    } else if h < 5.0/6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
