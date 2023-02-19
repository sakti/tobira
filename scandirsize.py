import os
import sys
import humanize


def get_size(start_path: str = ".") -> int:
    total_size = 0

    with os.scandir(start_path) as it:
        for entry in it:
            if entry.is_file():
                total_size += entry.stat().st_size
            elif entry.is_dir():
                total_size += get_size(entry.path)

    return total_size


def format(size: int) -> str:
    return humanize.naturalsize(size, binary=True)


def get_directory_size(path: str = ".") -> str:
    result = get_size(path)
    return format(result)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("need path argument")
        sys.exit(1)

    result = get_directory_size(sys.argv[1])
    print(result)
