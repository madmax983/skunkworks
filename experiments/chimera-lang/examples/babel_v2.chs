grammar Math {
    rule main { number space op space number } -> { ?1 ?5 ?3 }
    token number { /\d+/ }

    token op { op_add | op_sub | op_mul | op_div }
    token op_add { "+" } -> { add }
    token op_sub { "-" } -> { sub }
    token op_mul { "*" } -> { mul }
    token op_div { "/" } -> { div }

    token space { /\s+/ }
}

strand main {
    "⚛️  Parsing '10 + 20' with Math grammar..." print
    polyglot Math { 10 + 20 }
    "Result (should be 30):" print
    print
}
