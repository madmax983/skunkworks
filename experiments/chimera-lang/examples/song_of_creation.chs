strand main {
    "Binding handler (strand 2) to 440Hz..." print
    440 2 listen_freq

    "Resonating at 440Hz..." print
    # Amplitude 50 (Cost 10)
    # 50 - 10 = 40 Energy left
    440 50 resonate

    "Waiting..." print
    0
    jump(wait_loop)
}

strand wait_loop {
    # Stack: [ cnt ]
    1 add
    dup

    # Gain energy (+5)
    photosynthesize

    # Check 20 ticks
    dup 20 sub brz(end_loop)

    jump(wait_loop)
}

strand handler {
    "HARMONY ACHIEVED! 🎵" print
    100 5 lumine
    ret
}

strand end_loop {
    "Simulation complete." print
    3 apoptosis
}
