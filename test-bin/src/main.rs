// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::HashMap,
    process::exit,
    time::{Duration, Instant},
};

use aria_compiler::compile_from_source;
use aria_parser::ast::SourceBuffer;
use clap::{command, Parser};
use glob::Paths;
use haxby_vm::{frame::Frame, vm::VirtualMachine};
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// A glob expression resulting in which test files to run
    #[arg(long)]
    path: String,
    #[arg(long)]
    /// Print additional output information
    verbose: bool,
    #[arg(long)]
    /// Run tests sequentially instead of in parallel
    sequential: bool,
}

enum TestCaseResult {
    Pass(Duration),
    Fail(String),
}

fn run_test_from_pattern(path: &str) -> TestCaseResult {
    let start = Instant::now();

    let buffer = match SourceBuffer::file(path) {
        Ok(buffer) => buffer,
        Err(err) => {
            let fail_msg = format!("I/O error: {err}");
            return TestCaseResult::Fail(fail_msg);
        }
    };

    let entry_cm = match compile_from_source(&buffer, &Default::default()) {
        Ok(m) => m,
        Err(e) => {
            let err_msg = e
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n");
            return TestCaseResult::Fail(format!("compilation error: {err_msg}"));
        }
    };

    let mut vm = VirtualMachine::default();
    let entry_rm = match vm.load_module("", entry_cm) {
        Ok(rle) => match rle {
            haxby_vm::vm::RunloopExit::Ok(m) => m.module,
            haxby_vm::vm::RunloopExit::Exception(e) => {
                let mut frame = Frame::default();
                let epp = e.value.prettyprint(&mut frame, &mut vm);
                return TestCaseResult::Fail(epp);
            }
        },
        Err(err) => {
            return TestCaseResult::Fail(err.prettyprint(None));
        }
    };

    match vm.execute_module(&entry_rm) {
        Ok(rle) => match rle {
            haxby_vm::vm::RunloopExit::Ok(_) => {
                let duration = start.elapsed();
                TestCaseResult::Pass(duration)
            }
            haxby_vm::vm::RunloopExit::Exception(e) => {
                let mut frame = Frame::default();
                let epp = e.value.prettyprint(&mut frame, &mut vm);
                TestCaseResult::Fail(epp)
            }
        },
        Err(err) => TestCaseResult::Fail(err.prettyprint(Some(entry_rm))),
    }
}

#[derive(Default)]
struct SuiteReport {
    passes: Vec<(String, Duration)>,
    fails: HashMap<String, String>,
    duration: Duration,
}

impl SuiteReport {
    fn num_fails(&self) -> usize {
        self.fails.len()
    }

    fn num_passes(&self) -> usize {
        self.passes.len()
    }

    fn len(&self) -> usize {
        self.num_fails() + self.num_passes()
    }

    fn pass(&mut self, name: &str, duration: &Duration) {
        self.passes.push((name.to_owned(), *duration));
    }

    fn fail(&mut self, name: &str, why: String) {
        self.fails.insert(name.to_owned(), why);
    }
}

fn run_tests_from_pattern(pattern: Paths, args: &Args) -> SuiteReport {
    let mut results = SuiteReport::default();

    let start = Instant::now();

    let outcomes = if args.sequential {
        pattern
            .flatten()
            .map(|path| {
                let test_name = path.file_stem().unwrap().to_str().unwrap();
                let test_path = path.as_os_str().to_str().unwrap();
                if args.verbose {
                    println!("Running {test_name} (at {test_path})");
                }
                (test_name.to_owned(), run_test_from_pattern(test_path))
            })
            .collect::<Vec<(String, TestCaseResult)>>()
    } else {
        pattern
            .flatten()
            .par_bridge()
            .map(|path| {
                let test_name = path.file_stem().unwrap().to_str().unwrap();
                let test_path = path.as_os_str().to_str().unwrap();
                (test_name.to_owned(), run_test_from_pattern(test_path))
            })
            .collect::<Vec<(String, TestCaseResult)>>()
    };

    results.duration = start.elapsed();

    for result in &outcomes {
        match &result.1 {
            TestCaseResult::Pass(duration) => results.pass(&result.0, duration),
            TestCaseResult::Fail(why) => {
                results.fail(&result.0, why.clone());
            }
        }
    }

    results
}

fn main() {
    let args = Args::parse();
    let results = match glob::glob(&args.path) {
        Ok(pattern) => run_tests_from_pattern(pattern, &args),
        Err(err) => {
            eprintln!("invalid pattern: {err}");
            exit(1);
        }
    };
    if results.num_fails() == 0 && !args.verbose {
        println!("All tests passed; --verbose to print full report");
        exit(0);
    }

    for pass in &results.passes {
        println!(
            "{} ✅ [in {}.{:03} seconds]",
            pass.0,
            pass.1.as_secs(),
            pass.1.subsec_millis()
        );
    }

    for fail in &results.fails {
        println!("{} ❌", fail.0);
        println!("   reason: {}", fail.1);
    }

    println!(
        "{} test(s) total - {} passed, {} failed - in {}.{:03} seconds",
        results.len(),
        results.num_passes(),
        results.num_fails(),
        results.duration.as_secs(),
        results.duration.subsec_millis(),
    );
}
