strand main {
    "Scanning current directory..." print
    "." crawl
    print

    "Creating payload..." print
    "payload.txt" "This is a benign viral payload." synthesize

    "Verifying payload..." print
    "payload.txt" sequencing print

    "Injecting extra code..." print
    "payload.txt" "\nAppended by Chimera." infect

    "Final verification..." print
    "payload.txt" sequencing print

    "Cleaning up tracks..." print
    "rm payload.txt" shell drop
    "Cleaned." print
}
