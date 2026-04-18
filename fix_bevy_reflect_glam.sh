#!/bin/bash
# Find the exact path for the bevy_reflect glam.rs file
GLAM_FILE=$(find ~/.cargo/registry/src/ -path "*bevy_reflect-0.14.2/src/impls/glam.rs" | head -n 1)
sed -i 's/impl_reflect_value!(::glam::BVec4A(Debug, Default));/\/\/ impl_reflect_value!(::glam::BVec4A(Debug, Default));/' "$GLAM_FILE"
