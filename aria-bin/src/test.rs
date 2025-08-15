// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, rc::Rc};

use haxby_vm::console::TestConsole;

use crate::{Args, repl_eval::Repl};

fn build_test_repl<'a>(cmdline_options: &'a Args) -> Repl<'a> {
    let console = Rc::new(RefCell::new(TestConsole::default()));
    let mut vm_options = haxby_vm::vm::VmOptions::default();
    vm_options.console = console.clone();
    Repl::new(vm_options, &cmdline_options).unwrap()
}

fn run_passing_repl_line(repl: &mut Repl, line: &str, must_include_stdout: &[&str]) {
    let diff = repl.eval_line(line);

    assert!(diff.ok);
    for expected in must_include_stdout {
        assert!(diff.stdout.contains(expected));
    }
}

#[test]
fn repl_can_print_integers() {
    let cmdline_options = Args::default();
    let mut repl = build_test_repl(&cmdline_options);

    run_passing_repl_line(&mut repl, "42;", &["42"]);
    run_passing_repl_line(&mut repl, "3 + 4;", &["7"]);
}

#[test]
fn repl_can_call_functions() {
    let cmdline_options = Args::default();
    let mut repl = build_test_repl(&cmdline_options);

    run_passing_repl_line(&mut repl, "func foo(x) { return x + 1; }", &[]);
    run_passing_repl_line(&mut repl, "foo(12);", &["13"]);
}

#[test]
fn repl_can_define_structs() {
    let cmdline_options = Args::default();
    let mut repl = build_test_repl(&cmdline_options);

    run_passing_repl_line(
        &mut repl,
        r#"
struct Pair {
        type func new(x,y) {
            return alloc(This) {
                .x = x, .y = y,
            };
        }
}
    "#,
        &[],
    );

    run_passing_repl_line(&mut repl, "val p = Pair.new(4,5);", &[]);

    run_passing_repl_line(
        &mut repl,
        r#"
extension Pair {
        func prettyprint() {
            return "Pair({0},{1})".format(this.x,this.y);
        }
}
    "#,
        &[],
    );

    run_passing_repl_line(&mut repl, "p;", &["Pair(4,5)"]);
}

#[test]
fn repl_can_use_if_statement() {
    let cmdline_options = Args::default();
    let mut repl = build_test_repl(&cmdline_options);

    run_passing_repl_line(&mut repl, "val x = 4;", &[]);
    run_passing_repl_line(&mut repl, "if (x > 2) { println(x + 1); }", &["5"]);
    run_passing_repl_line(&mut repl, "if (x > 4) { println(x + 1); }", &[""]);
}

#[test]
fn repl_can_use_for_statement() {
    let cmdline_options = Args::default();
    let mut repl = build_test_repl(&cmdline_options);

    run_passing_repl_line(&mut repl, "val l = [1,2,3,4,5,6];", &[]);
    run_passing_repl_line(
        &mut repl,
        "for i in l { println(i); }",
        &["1", "2", "3", "4", "5", "6"],
    );
}

#[test]
fn repl_can_use_while_statement() {
    let cmdline_options = Args::default();
    let mut repl = build_test_repl(&cmdline_options);

    run_passing_repl_line(&mut repl, "val n = 10;", &[]);
    run_passing_repl_line(
        &mut repl,
        "while n > 0 { n -= 2; println(n); }",
        &["8", "6", "4", "2", "0"],
    );
}

#[test]
fn repl_can_use_match_statement() {
    let cmdline_options = Args::default();
    let mut repl = build_test_repl(&cmdline_options);

    run_passing_repl_line(&mut repl, "val n = 10;", &[]);

    run_passing_repl_line(
        &mut repl,
        r#"
match n {
        > 10 => {
            println("Greater than 10");
        }
} else {
        println("10 or less");
}
"#,
        &["10 or less"],
    );
}
