#include <cstdio>
#include <cstdlib>
#include <filesystem>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>
#define ANKERL_NANOBENCH_IMPLEMENT
#include "/home/borja/projects/aria/bench-bin/deps/nanobench/src/include/nanobench.h"

using namespace ankerl::nanobench;

// Helper: split string by delimiter
std::vector<std::filesystem::path> split(const std::string &s, char delim) {
  std::vector<std::filesystem::path> elems;
  std::stringstream ss(s);
  std::string item;
  while (std::getline(ss, item, delim)) {
    if (!item.empty()) {
      elems.push_back(std::filesystem::path(item));
    }
  }
  return elems;
}

void exec_bench_on_file(Bench bench, std::filesystem::path src);

int main(int argc, char *argv[]) {

  if (argc < 2) {
    std::cout << "Usage: " << argv[0] << " <dir1:dir2:...>" << std::endl;
    return 1;
  }

  std::string pattern = "";
  if (argc > 2) {
    pattern = argv[2];
  }

  std::vector<std::filesystem::path> bench_dirs =
      split(std::string(argv[1]), ':');

  Bench bench = Bench();

  for (const std::filesystem::path dir : bench_dirs) {
    for (const auto &entry : std::filesystem::directory_iterator(dir)) {
      if (entry.is_regular_file() &&
          entry.path().filename().string().find(pattern) != std::string::npos &&
          entry.path().extension().string() == ".aria") {
        exec_bench_on_file(bench, entry.path());
      }
    }
  }
}

void exec_bench_on_file(Bench bench, std::filesystem::path src) {
  std::string command_string = "./target/release/aria " + src.string();
  const char *command = command_string.c_str();

  bench.run(src.c_str(), [&] { int _ = std::system(command); });
}
