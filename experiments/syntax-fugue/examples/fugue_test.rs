fn main_theme() {
    loop {
        let rhythm = 1;
        let beat = 2;
        if rhythm > beat {
            break;
        }
    }
}

fn counter_subject() {
    let mut melody = 0;
    while melody < 10 {
        melody += 1;
        match melody {
            1 => { let _note = "C"; },
            2 => { let _note = "D"; },
            _ => { let _note = "E"; },
        }
    }
}

fn finale() {
    main_theme();
    counter_subject();
    let _end = true;
}

fn main() {
    finale();
}
