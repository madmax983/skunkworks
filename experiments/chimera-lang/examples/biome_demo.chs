strand main {
    # Boost Energy for Terraforming God Mode
    push(10000) consume

    "Terraforming Grid..." print

    # Center (8,8): Garden (ID 8, Radius 6)
    push(8) push(6) terraform

    # Move to Top Left (2,2)
    push(-6) push(-6) migrate

    # Terraform Volcanic (ID 4, Radius 3)
    push(4) push(3) terraform

    # Spawn Fireborn
    # Stack: [fireborn_idx, type_0]
    push(fireborn) push(0) spawn

    # Move to Bottom Right (13,13)
    push(11) push(11) migrate

    # Terraform Glitch (ID 5, Radius 3)
    push(5) push(3) terraform

    # Spawn Glitcher
    push(glitcher) push(0) spawn

    "Biomes Established." print

    jump(main_loop)
}

strand main_loop {
    push(1) consume
    jump(main_loop)
}

strand fireborn {
    "Fireborn spawned." print
    sense_biome
    # Check if Volcanic (4)
    push(4) sub
    # If 0 (Equal), jump to thrive
    brz(fireborn_thrive)

    # Else Die
    "Fireborn dies in cold." print
    apoptosis
}

strand fireborn_thrive {
    "Fireborn thrives in heat." print
    jump(fireborn_live)
}

strand fireborn_live {
    push(1) consume
    jump(fireborn_live)
}

strand glitcher {
    "Glitcher spawned." print
    sense_biome
    jump(glitcher_live)
}

strand glitcher_live {
    push(1) consume
    jump(glitcher_live)
}
