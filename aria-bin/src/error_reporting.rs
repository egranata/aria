// SPDX-License-Identifier: Apache-2.0
use std::{collections::HashMap, vec};

use aria_compiler::do_compile::CompilationError;
use aria_parser::ast::{ParserError, SourcePointer};
use ariadne::{Color, Label, Report, ReportKind, Source};
use haxby_vm::{
    error::{exception::VmException, vm_error::VmError},
    vm::VirtualMachine,
};

#[derive(Default, Debug, Clone)]
pub struct StringCache {
    buffers: HashMap<String, Source>,
}

impl ariadne::Cache<String> for StringCache {
    type Storage = String;

    fn fetch(
        &mut self,
        path: &String,
    ) -> Result<&Source<<Self as ariadne::Cache<String>>::Storage>, impl std::fmt::Debug> {
        Ok::<&Source, Source>(&self.buffers[path])
    }

    #[allow(refining_impl_trait)]
    fn display<'a>(&self, path: &'a String) -> Option<impl std::fmt::Display + 'a> {
        Some(Box::new((*path).clone()))
    }
}

pub(crate) type PrintableReport<'a> = (
    ariadne::Report<'a, (std::string::String, std::ops::Range<usize>)>,
    StringCache,
);

fn build_report_from_msg_and_location<'a>(
    msg: &str,
    locations: Vec<SourcePointer>,
) -> PrintableReport<'a> {
    let config = ariadne::Config::default().with_index_type(ariadne::IndexType::Byte);
    let magenta = Color::Magenta;
    let mut report = Report::build(ReportKind::Error, ("unknown".to_owned(), 0..0))
        .with_message(msg)
        .with_config(config);
    let mut cache = StringCache::default();
    for (idx, loc) in locations.iter().enumerate() {
        let loc = loc.clone();
        report = report.with_label(
            Label::new((
                loc.buffer.name.clone(),
                loc.location.start..loc.location.stop,
            ))
            .with_message("here")
            .with_order(idx as i32)
            .with_color(magenta),
        );
        if !cache.buffers.contains_key(&loc.buffer.name) {
            cache.buffers.insert(
                loc.buffer.name.clone(),
                Source::from((*loc.buffer.content).clone()),
            );
        }
    }
    (report.finish(), cache)
}

pub(crate) fn print_report_from_vm_exception(vm: &mut VirtualMachine, exc: &VmException) {
    let (report, cache) = build_report_from_vm_exception(vm, exc);
    report.eprint(cache).unwrap();
}

pub(crate) fn print_report_from_compiler_error(err: &CompilationError) {
    let (report, cache) = build_report_from_compiler_error(err);
    report.eprint(cache).unwrap();
}

pub(crate) fn print_report_from_parser_error(err: &ParserError) {
    let (report, cache) = build_report_from_parser_error(err);
    report.eprint(cache).unwrap();
}

pub(crate) fn print_report_from_vm_error(err: &VmError) {
    let (report, cache) = build_report_from_vm_error(err);
    report.eprint(cache).unwrap();
}

pub(crate) fn build_report_from_vm_error<'a>(err: &VmError) -> PrintableReport<'a> {
    let msg = err.reason.to_string();
    if err.backtrace.is_empty() {
        if let Some(loc) = &err.loc {
            build_report_from_msg_and_location(&msg, vec![loc.clone()])
        } else {
            build_report_from_msg_and_location(&msg, vec![])
        }
    } else {
        let backtraces: Vec<_> = err.backtrace.entries_iter().cloned().collect();
        build_report_from_msg_and_location(&msg, backtraces)
    }
}

pub(crate) fn build_report_from_vm_exception<'a>(
    vm: &mut VirtualMachine,
    exc: &'a VmException,
) -> PrintableReport<'a> {
    let mut cur_frame = Default::default();
    let msg = exc.value.prettyprint(&mut cur_frame, vm);
    let backtraces: Vec<_> = exc.backtrace.entries_iter().cloned().collect();
    build_report_from_msg_and_location(&msg, backtraces)
}

pub(crate) fn build_report_from_compiler_error<'a>(
    err: &'a CompilationError,
) -> PrintableReport<'a> {
    let msg = err.reason.to_string();
    let loc = &err.loc;
    build_report_from_msg_and_location(&msg, vec![loc.clone()])
}

pub(crate) fn build_report_from_parser_error<'a>(err: &'a ParserError) -> PrintableReport<'a> {
    let msg = &err.msg;
    let loc = &err.loc;
    build_report_from_msg_and_location(msg, vec![loc.clone()])
}
