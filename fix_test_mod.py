import re

with open("experiments/chimera-lang/src/prolouge_compiler.rs", "r") as f:
    content = f.read()

test_mod_start = content.find("#[cfg(feature = \"nova\")]\n#[cfg(test)]\nmod tests {")
if test_mod_start != -1:
    content_before = content[:test_mod_start]
    content_after_start = content[test_mod_start:]

    # find matching brace
    brace_count = 0
    test_mod_end = -1
    in_mod = False
    for i, char in enumerate(content_after_start):
        if char == '{':
            if not in_mod:
                in_mod = True
            brace_count += 1
        elif char == '}':
            brace_count -= 1
            if brace_count == 0 and in_mod:
                test_mod_end = i
                break

    if test_mod_end != -1:
        test_mod_content = content_after_start[:test_mod_end + 1]
        content_after_end = content_after_start[test_mod_end + 1:]

        # move test_mod_content to the end
        new_content = content_before + content_after_end + "\n\n" + test_mod_content

        with open("experiments/chimera-lang/src/prolouge_compiler.rs", "w") as f:
            f.write(new_content)
        print("Moved test module to the end.")
