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
#include "nanobench.h"
#include "nlohmann/json.hpp"

using namespace ankerl::nanobench;
using json = nlohmann::json;

// TODO: Show the error in the benchmarks comparison
// TODO: To much hardcoded stuff

void show_comparison_with_baseline();

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

  show_comparison_with_baseline();

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

/********************************
 * Print comparison here with the last baseline
 ********************************/

struct Measurement {
  int iterations;
  double elapsed;
  int pagefaults;
  long long cpucycles;
  int contextswitches;
  long long instructions;
  long long branchinstructions;
  long long branchmisses;
};

struct BenchmarkResult {
  std::string title;
  std::string name;
  std::string unit;
  int batch;
  int complexityN;
  int epochs;
  double clockResolution;
  int clockResolutionMultiple;
  double maxEpochTime;
  double minEpochTime;
  int minEpochIterations;
  int epochIterations;
  int warmup;
  int relative;
  double median_elapsed;
  double medianAbsolutePercentError_elapsed;
  double median_instructions;
  double medianAbsolutePercentError_instructions;
  double median_cpucycles;
  double median_contextswitches;
  double median_pagefaults;
  double median_branchinstructions;
  double median_branchmisses;
  double totalTime;
  std::vector<Measurement> measurements;
};

struct BenchmarkData {
  std::vector<BenchmarkResult> results;
};

void from_json(const json &j, Measurement &m) {
  j.at("iterations").get_to(m.iterations);
  j.at("elapsed").get_to(m.elapsed);
  j.at("pagefaults").get_to(m.pagefaults);
  j.at("cpucycles").get_to(m.cpucycles);
  j.at("contextswitches").get_to(m.contextswitches);
  j.at("instructions").get_to(m.instructions);
  j.at("branchinstructions").get_to(m.branchinstructions);
  j.at("branchmisses").get_to(m.branchmisses);
}

void from_json(const json &j, BenchmarkResult &r) {
  j.at("title").get_to(r.title);
  j.at("name").get_to(r.name);
  j.at("unit").get_to(r.unit);
  j.at("batch").get_to(r.batch);
  j.at("complexityN").get_to(r.complexityN);
  j.at("epochs").get_to(r.epochs);
  j.at("clockResolution").get_to(r.clockResolution);
  j.at("clockResolutionMultiple").get_to(r.clockResolutionMultiple);
  j.at("maxEpochTime").get_to(r.maxEpochTime);
  j.at("minEpochTime").get_to(r.minEpochTime);
  j.at("minEpochIterations").get_to(r.minEpochIterations);
  j.at("epochIterations").get_to(r.epochIterations);
  j.at("warmup").get_to(r.warmup);
  j.at("relative").get_to(r.relative);

  // campos con nombres raros, como "median(elapsed)"
  r.median_elapsed = j.value("median(elapsed)", 0.0);
  r.medianAbsolutePercentError_elapsed =
      j.value("medianAbsolutePercentError(elapsed)", 0.0);
  r.median_instructions = j.value("median(instructions)", 0.0);
  r.medianAbsolutePercentError_instructions =
      j.value("medianAbsolutePercentError(instructions)", 0.0);
  r.median_cpucycles = j.value("median(cpucycles)", 0.0);
  r.median_contextswitches = j.value("median(contextswitches)", 0.0);
  r.median_pagefaults = j.value("median(pagefaults)", 0.0);
  r.median_branchinstructions = j.value("median(branchinstructions)", 0.0);
  r.median_branchmisses = j.value("median(branchmisses)", 0.0);
  r.totalTime = j.value("totalTime", 0.0);

  // measurements
  if (j.contains("measurements"))
    j.at("measurements").get_to(r.measurements);
}

void from_json(const json &j, BenchmarkData &d) {
  j.at("results").get_to(d.results);
}

void show_comparison_with_baseline() {
  std::ifstream baseline_f("target/nanobench/baseline/mustache.render.json");

  if (!baseline_f.is_open()) {
    return;
  }

  json baseline_json = json::parse(baseline_f);
  BenchmarkData baseline_data = baseline_json.get<BenchmarkData>();

  std::ifstream new_f("target/nanobench/results/mustache.render.json");
  json new_json = json::parse(new_f);
  BenchmarkData new_data = new_json.get<BenchmarkData>();

  bool any_diff = false;
  const float threshold = 0.05;

  printf("\n\n");
  printf("| Baseline (s)  | New (s) |   Δ%%    | Benchmark\n");
  printf("|---------------|---------|---------|--------------------------\n");

  for (size_t i = 0;
       i < baseline_data.results.size() && i < new_data.results.size(); ++i) {
    const auto &base = baseline_data.results[i];
    const auto &curr = new_data.results[i];

    double old_val = base.median_elapsed;
    double new_val = curr.median_elapsed;

    if (old_val <= 0.0) {
      continue;
    }

    double diff = (new_val - old_val) / old_val;
    double diff_percent = diff * 100.0;

    if (std::fabs(diff) < threshold) {
      continue;
    }

    any_diff = true;

    const char *color = (diff < 0) ? "\033[1;92m"  // shiny green
                                   : "\033[1;91m"; // shiny red

    const char *reset = "\033[0m";

    printf("| %10.6f | %10.6f | %s%+6.2f%%%s | %s \n", old_val, new_val, color,
           diff_percent, reset, base.name.c_str());
  }

  if (!any_diff) {
    printf("\n✅ No variations detected \n");
  }
}
