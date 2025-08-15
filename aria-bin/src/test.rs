// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, rc::Rc};

use haxby_vm::console::TestConsole;

use crate::{Args, repl_eval::Repl};

#[test]
fn repl_can_print_integers() {
    let console = Rc::new(RefCell::new(TestConsole::default()));
    let mut vm_options = haxby_vm::vm::VmOptions::default();
    vm_options.console = console.clone();
    let cmdline_options = Args::default();
    let mut repl = Repl::new(vm_options, &cmdline_options).unwrap();

    assert!(repl.process_buffer("42;").is_ok());
    assert!(console.borrow().stdout == "42\n");

    console.borrow_mut().clear();
    assert!(repl.process_buffer("3 + 4;").is_ok());
    assert!(console.borrow().stdout == "7\n");
}
