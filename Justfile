all:
    just --list

run_python_module:
    python calculate.py ~/dev/linux-6.1.11

run_rust_module:
    python calculate.py -rust ~/dev/linux-6.1.11

benchmark:
    hyperfine --warmup 1 -r 20 "python calculate.py ~/dev/linux-6.1.11" "python calculate.py -rust ~/dev/linux-6.1.11" --export-json result.json 

single_benchmark:
    hyperfine --warmup 1 "python calculate.py ~/dev/linux-6.1.11" --export-json result.json 

http:
    python -m http.server

graph:
    python plot_histogram.py -o histogram.png result.json
    python plot_whisker.py -o whisker.png result.json

graph_bins:
    python plot_histogram.py --bins 100 -o histogram.png result.json
