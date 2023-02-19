import argparse

import tobira  # rust module
import scandirsize  # python module

parser = argparse.ArgumentParser(description="Compare directory size module")
parser.add_argument("path")
parser.add_argument("-rust", "--rust-module", action="store_true")


if __name__ == "__main__":
    args = parser.parse_args()
    if args.rust_module:
        result = tobira.get_directory_size(args.path)
    else:
        result = scandirsize.get_directory_size(args.path)
    print(result)
