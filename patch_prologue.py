import sys

def modify_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Optimized version with Swap
    old_prepare = """fn prepare_signals(vm: &mut ChimeraVM, grid: &[Vec<Value>]) {
    let mut current_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if let Some(val) = &vm.prologue_state.delayed_signals[y][x] {
                current_signals[y][x] = Some(val.clone());
            }
        }
    }
    vm.prologue_state.signal_grid = current_signals;
    vm.prologue_state.delayed_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];"""

    new_prepare = """fn prepare_signals(vm: &mut ChimeraVM, grid: &[Vec<Value>]) {
    if vm.prologue_state.scratch_signal_grid.len() != GRID_SIZE {
        vm.prologue_state.scratch_signal_grid = vec![vec![None; GRID_SIZE]; GRID_SIZE];
    }
    let mut current_signals = std::mem::take(&mut vm.prologue_state.scratch_signal_grid);

    for (y, row) in current_signals.iter_mut().enumerate().take(GRID_SIZE) {
        for (x, cell) in row.iter_mut().enumerate().take(GRID_SIZE) {
            if let Some(val) = &vm.prologue_state.delayed_signals[y][x] {
                *cell = Some(val.clone());
            } else {
                *cell = None;
            }
        }
    }
    std::mem::swap(&mut vm.prologue_state.signal_grid, &mut current_signals);
    vm.prologue_state.scratch_signal_grid = current_signals;

    for row in vm.prologue_state.delayed_signals.iter_mut() {
        row.fill(None);
    }"""

    if old_prepare in content:
        content = content.replace(old_prepare, new_prepare)

    # Let's ALSO optimize propagate carefully.
    old_propagate = """fn process_signal_propagation(vm: &mut ChimeraVM, grid: &[Vec<Value>]) {
    let max_iterations = GRID_SIZE * 2;
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();
    let tick = vm.tick_counter;

    if vm.prologue_state.scratch_signal_grid.len() != GRID_SIZE {
        vm.prologue_state.scratch_signal_grid = vec![vec![None; GRID_SIZE]; GRID_SIZE];
    }

    for _ in 0..max_iterations {
        let mut changes = false;
        let mut next_signals = std::mem::take(&mut vm.prologue_state.scratch_signal_grid);
        if next_signals.len() != GRID_SIZE {
            next_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];
        }
        next_signals.clone_from(&vm.prologue_state.signal_grid);"""

    new_propagate = """fn process_signal_propagation(vm: &mut ChimeraVM, grid: &[Vec<Value>]) {
    let max_iterations = GRID_SIZE * 2;
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();
    let tick = vm.tick_counter;

    if vm.prologue_state.scratch_signal_grid.len() != GRID_SIZE {
        vm.prologue_state.scratch_signal_grid = vec![vec![None; GRID_SIZE]; GRID_SIZE];
    }

    for _ in 0..max_iterations {
        let mut changes = false;
        let mut next_signals = std::mem::take(&mut vm.prologue_state.scratch_signal_grid);
        if next_signals.len() != GRID_SIZE {
            next_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];
        }
        for (y, row) in next_signals.iter_mut().enumerate().take(GRID_SIZE) {
            row.clone_from_slice(&vm.prologue_state.signal_grid[y]);
        }"""

    if old_propagate in content:
        content = content.replace(old_propagate, new_propagate)

    with open(filepath, 'w') as f:
        f.write(content)

modify_file('experiments/chimera-lang/src/vm/prologue/mod.rs')
