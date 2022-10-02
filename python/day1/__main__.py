
from base import read_input


def main():
    input = read_input(1)[0]  # Get first line
    ups = input.count("(")
    downs = input.count(")")

    print(f"Part 1: { ups - downs }")

    res = 0
    for k, c in enumerate(input, 1):
        res += 1 if c == "(" else -1
        if res == -1:
            break

    print(f"Part 1: {k}")


if __name__ == "__main__":
    main()
