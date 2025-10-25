# Aria Benchmarking Guide

In this guide, we will cover the following topics:

- [Setting up the environment](setting-up-the-environment)
- [Exploring the benchmarks frameworks we are using](the-benchmarks)

## Setting up the environment

For obvious reasons, we need Rust installed on our system in order to run the benchmarks. This is the only strict requirement, but if you experience unstable results, we recommend following as many of these suggestions as possible:

- Use `taskset` to always run the benchmarks on the same CPU cores.
- Disable features such as `hyperthreading` and `turbo boost`.
- Use a fixed CPU frequency for the cores running the benchmarks.
- Isolate the CPU cores being used so that other processes do not interfere.

Most of these changes can easily be applied using [`pyperf`](https://github.com/psf/pyperf) by running the following command:

```bash
pyperf system tune
```

This command locks CPU frequency, disables turbo boost, and applies other system-level adjustments to improve benchmark stability. Don’t worry about the changes being permanent, they are temporary and will be reverted when you reboot your system.

Core isolation, however, must be done manually if you need it.

```
Note: These are only recommendations and may not be applicable to all systems or use cases. Running the benchmarks with your normal machine configuration should still yield stable enough results, as Criterion, the benchmark framework we use for Rust, is reliable.
```

```
Important: These changes aim to make benchmarking more stable and accurate. They do not improve the execution speed or performance of your code or machine — quite the opposite, they often slow everything down slightly. The goal is to detect real performance improvements or regressions caused by code changes.
```

## The benchmarks

As mentioned before, the framework we use to benchmark the Rust code is [`Criterion`](https://crates.io/crates/criterion). For detailed information on how to create benchmarks with it, please refer to its official documentation.

To execute benchmarks in Aria, we recommend using the `b` script located at the root of the repository.
It provides a convenient interface for building and running benchmarks, handling paths, and ensuring Aria can locate the libraries.

### Usage of the `b` script

```bash
./b $COMMAND $BENCH
```

`b` takes two optional arguments:

- `command` (required): One of the following:. 
    - `bench` runs the benchmarks using Criterion. This is what most users will use to measure the performance of the code they are developing or optimizing.
    - `perf` profiles execution using Linux’s `perf` tool.
    - `valgrind` runs under Valgrind’s `callgrind` tool to analyze performance.
    - `time` measures wall time, user time, and system time for a single run. Useful for quick checks, but only reliable if your environment is properly tuned.
- `bench` (optional): The name or partial name of the benchmark you want to run. If omitted, all benchmarks will be executed, which may take a long time and is usually not what you want. If provided, only benchmarks whose names contain the given string will be executed.
