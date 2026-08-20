def fix_universe():
    with open('experiments/ferrous-chimera/src/universe.rs', 'r') as f:
        content = f.read()
    content = content.replace('rand::random::<u8>() % 5 == 0', 'rand::random::<u8>().is_multiple_of(5)')
    with open('experiments/ferrous-chimera/src/universe.rs', 'w') as f:
        f.write(content)

fix_universe()
