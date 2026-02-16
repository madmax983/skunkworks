strand main {
    # 1. Define Payload (What the virus does)
    "VIRUS PAYLOAD EXECUTED" print

    # 2. Conceive Meme (Capture above code)
    # Length: 2 (String + Print)
    # Virulence: 100 (Always infect)
    # Fidelity: 100 (No mutation)
    100 100 2 conceive
    # Stack: [ meme_id ]

    # 3. Prepare BioHack Arguments
    # We need: [ Name, Grammar, MemeID ]

    # Push Name (Bottom of args)
    "ChimeraOmega"

    # Swap Name and MemeID
    swap
    # Stack: [ name, meme_id ]

    # Define Grammar (For host mutation)
    # Generate "OBEY"
    "OBEY" grammar(Match)
    # Stack: [ name, meme_id, grammar ]

    # Swap Grammar and MemeID to get MemeID on top
    swap
    # Stack: [ name, grammar, meme_id ]

    # 4. Execute BioHack
    bio-hack

    "Virus Synthesized and Released." print

    # 5. Verify infection (Visual check in TUI or logic)
    # Check viral grid at current location?
    # (No opcode to read viral grid directly yet, but we can assume it worked)
}
