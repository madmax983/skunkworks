for dir in crates/* experiments/*; do
  if [ -f "$dir/Cargo.toml" ]; then
    crate=$(basename $dir)
    echo "Checking $crate..."
    RUSTDOCFLAGS="-W missing_docs -W rustdoc::missing_crate_level_docs -D warnings" cargo doc -p $crate --no-deps >/dev/null 2>&1
    if [ $? -ne 0 ]; then
      echo "Failed: $crate"
    fi
  fi
done
