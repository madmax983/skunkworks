sed -i 's/_ = hovered_node;/let mut hovered_node: Option<usize> = None;/g' graveyard/mnem-hologram/src/main.rs
git restore graveyard/mnem-hologram/src/hologram.rs
rm fix_other_enums.py fix_plucks.py fix_clockwork.py fix_clockwork.sh fix_dead_code.py fix_syntax.py
