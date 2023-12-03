def run_program(program):
    ptr = 0
    while True:
        v = program[ptr]
        if v == 99:
            return program

        a = program[program[ptr + 1]]
        b = program[program[ptr + 2]]
        store_at = program[ptr + 3]
        if v == 1:
            result = a + b
        elif v == 2:
            result = a * b
        else:
            print(f"INVALID OPCODE {v} @ {ptr}")
            return [-1]
        program[store_at] = result
        ptr += 4


def do_test_cases():
    for test, expected in [
        ("1,0,0,0,99", "2,0,0,0,99"),
        ("2,3,0,3,99", "2,3,0,6,99"),
        ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
        ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
    ]:
        program = [int(x) for x in test.split(",")]
        result = ",".join(str(x) for x in run_program(program))
        pass_or_fail = "PASS" if result == expected else "FAIL"
        print(f"{pass_or_fail}: {test}  ->  {result}")


def do_real():
    with open("./input.txt", "r") as f:
        while True:
            program = f.readline().strip()
            if not program:
                break
            program = [int(x) for x in program.split(",")]
            # Restore 1202 alarm
            program[1] = 12
            program[2] = 2

            print(run_program(program)[0])


# do_test_cases()
do_real()
