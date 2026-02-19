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
            1 => { let note = "C"; },
            2 => { let note = "D"; },
            _ => { let note = "E"; },
        }
    }
}

fn finale() {
    main_theme();
    counter_subject();
    let end = true;
}
