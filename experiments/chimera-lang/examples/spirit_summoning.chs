strand main {
    "Invoking the Spirit..." print

    # Pause and ask user for input
    spirit

    # Check if input matches 42
    dup
    print
    42 sub
    brz(success)

    "The Spirit has spoken, but the answer is mundane." print
    apoptosis
}

strand success {
    "The Spirit reveals the Ultimate Truth!" print

    # Sing the Song of Genesis
    "Do" sing
    "Mi" sing
    "Sol" sing

    # Wait for the song to take effect
    5 chronostasis

    "Life created." print
}
