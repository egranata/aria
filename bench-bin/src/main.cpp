#include <cstdio>
#include <cstdlib>
#include <filesystem>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>
#include <unistd.h>
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

void exec_bench_on_file(Bench &bench, std::filesystem::path src) {
  std::string command_string = "./target/release/aria " + src.string();
  const char *command = command_string.c_str();

  bench.run(src.c_str(), [&] { int _ = std::system(command); });
}

void write_output(std::filesystem::path folder, std::string const &typeName,
                  char const *mustacheTemplate, Bench const &bench) {
  std::filesystem::create_directories(folder);

  std::ofstream resultsOut(folder / ("mustache.render." + typeName));

  ankerl::nanobench::render(mustacheTemplate, bench, resultsOut);
}

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

  bench.title("Benchmarking Aria execution time");

  for (const std::filesystem::path dir : bench_dirs) {
    for (const auto &entry : std::filesystem::directory_iterator(dir)) {
      if (entry.is_regular_file() &&
          entry.path().filename().string().find(pattern) != std::string::npos &&
          entry.path().extension().string() == ".aria") {
        exec_bench_on_file(bench, entry.path());
      }
    }
  }

  std::filesystem::path results_folder = "target/nanobench/results";

  write_output(results_folder, "html", templates::htmlBoxplot(), bench);
  write_output(results_folder, "csv", templates::csv(), bench);
  write_output(results_folder, "json", templates::json(), bench);

  // Print comparison here with the last baseline

  // Ask if the user want to udpate the baseline results with the current
  // results

  printf("\nDo you want to save the new results as the next baseline?? [y/N] ");

  std::string input;
  std::getline(std::cin, input);

  if (!input.empty() && (input[0] == 'y' || input[0] == 'Y')) {
    std::filesystem::path baseline_folder = "target/nanobench/baseline";

    write_output(baseline_folder, "csv", templates::csv(), bench);
    write_output(baseline_folder, "json", templates::json(), bench);
    write_output(baseline_folder, "html", templates::htmlBoxplot(), bench);
  }

  return 0;
}
