import hashlib
import time
from base import read_input


def find_hash_integer(input: str, difficulty: int) -> int:
    needle = "0"*difficulty

    k = 1

    seed = f"{input}{k}".encode()
    hash = hashlib.md5(seed).digest().hex()
    while not hash.startswith(needle):
        k += 1
        seed = f"{input}{k}".encode()
        hash = hashlib.md5(seed).digest().hex()

    return k


def main():

    input_lines = read_input(4)
    input = input_lines[0]

    difficulty = 5
    tic = time.time()
    k = find_hash_integer(input, difficulty)
    eta = time.time() - tic
    print(f"Part 1: { k }")
    print(f"Elapsed time: {eta:.2f}")

    difficulty = 6
    tic = time.time()
    k = find_hash_integer(input, difficulty)
    eta = time.time() - tic
    print(f"Part 2: { k }")
    print(f"Elapsed time: {eta:.2f}")


if __name__ == "__main__":
    main()
