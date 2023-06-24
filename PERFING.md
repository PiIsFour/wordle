
# Install

```
apt-get install libdwarf-dev libdw-dev # dwarf
apt install flex bison
git clone https://github.com/microsoft/WSL2-Linux-Kernel --depth 1
cd WSL2-Linux-Kernel/tools/perf
make -j8
sudo cp perf /usr/local/bin
```

# Run

```
perf record --call-graph=dwarf ./target/release/wordle
perf script -F +pid > test.perf
```

# Viewing

https://profiler.firefox.com
