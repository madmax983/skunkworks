import re

with open("crates/tui-shared/src/semantic/snapshot.rs", "r") as f:
    content = f.read()

# Replace HashMap with BTreeMap
content = content.replace("std::collections::HashMap", "std::collections::BTreeMap")
content = content.replace("HashMap<", "BTreeMap<")
content = content.replace("HashMap::", "BTreeMap::")

search_block = """    pub fn new(app: impl Into<String>) -> Self {
        Self {
            app: app.into(),
            frame: None,
            viewport: None,
            entities: Vec::new(),
            regions: Vec::new(),
            metrics: BTreeMap::new(),
            state: None,
            actions: Vec::new(),
        }
    }"""

replace_block = """    pub fn new(app: impl Into<String>) -> Self {
        Self {
            app: app.into(),
            frame: None,
            viewport: None,
            // ⚡ Bolt: Pre-allocate vectors with typical capacities to reduce heap reallocations.
            entities: Vec::with_capacity(32),
            regions: Vec::with_capacity(8),
            // ⚡ Bolt: Use BTreeMap instead of HashMap for smaller structures (like properties and metrics)
            // to avoid the memory/hashing overhead of the default SipHasher, while gaining deterministic serialization.
            metrics: BTreeMap::new(),
            state: None,
            actions: Vec::with_capacity(8),
        }
    }"""

content = content.replace(search_block, replace_block)

with open("crates/tui-shared/src/semantic/snapshot.rs", "w") as f:
    f.write(content)

with open("crates/tui-shared/src/semantic/entity.rs", "r") as f:
    content = f.read()

# Replace HashMap with BTreeMap
content = content.replace("std::collections::HashMap", "std::collections::BTreeMap")
content = content.replace("HashMap<", "BTreeMap<")
content = content.replace("HashMap::", "BTreeMap::")

search_block2 = """    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            id: None,
            position: None,
            velocity: None,
            display: None,
            props: BTreeMap::new(),
        }
    }"""

replace_block2 = """    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            id: None,
            position: None,
            velocity: None,
            display: None,
            // ⚡ Bolt: Use BTreeMap instead of HashMap for smaller structures (like properties and metrics)
            // to avoid the memory/hashing overhead of the default SipHasher, while gaining deterministic serialization.
            props: BTreeMap::new(),
        }
    }"""

content = content.replace(search_block2, replace_block2)

with open("crates/tui-shared/src/semantic/entity.rs", "w") as f:
    f.write(content)
