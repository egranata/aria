// SPDX-License-Identifier: Apache-2.0
use std::collections::HashMap;

use aria_compiler::do_compile::CompilationError;
use aria_parser::ast::{ParserError, SourcePointer};
use ariadne::{Color, Label, Report, ReportKind, Source};
use haxby_vm::{
    error::{exception::VmException, vm_error::VmError},
    frame::Frame,
    vm::VirtualMachine,
};

#[derive(Default, Debug, Clone)]
pub struct StringCache {
    buffers: HashMap<String, Source>,
}

impl ariadne::Cache<&String> for StringCache {
    type Storage = String;

    fn fetch(
        &mut self,
        path: &&String,
    ) -> Result<
        &Source<<Self as ariadne::Cache<&std::string::String>>::Storage>,
        impl std::fmt::Debug,
    > {
        Ok::<&Source, Source>(&self.buffers[*path])
    }

    #[allow(refining_impl_trait)]
    fn display<'a>(&self, path: &&'a String) -> Option<impl std::fmt::Display + 'a> {
        Some(Box::new((**path).clone()))
    }
}

fn report_from_msg_and_location(msg: &str, locations: &[&SourcePointer]) {
    let config = ariadne::Config::default().with_index_type(ariadne::IndexType::Byte);
    let magenta = Color::Magenta;
    let primary_span = &locations[0];
    let mut report = Report::build(
        ReportKind::Error,
        (
            &primary_span.buffer.name,
            primary_span.location.start..primary_span.location.stop,
        ),
    )
    .with_message(msg)
    .with_config(config);
    let mut cache = StringCache::default();
    for loc in locations {
        report = report.with_label(
            Label::new((&loc.buffer.name, loc.location.start..loc.location.stop))
                .with_message("here")
                .with_color(magenta),
        );
        if !cache.buffers.contains_key(&loc.buffer.name) {
            cache.buffers.insert(
                loc.buffer.name.clone(),
                Source::from((*loc.buffer.content).clone()),
            );
        }
    }
    report.finish().eprint(cache).unwrap();
}

pub(crate) fn report_from_vm_error(err: &VmError) {
    let msg = err.reason.to_string();
    if err.backtrace.is_empty() {
        if let Some(loc) = &err.loc {
            report_from_msg_and_location(&msg, &[loc]);
        } else {
            eprintln!("vm execution error: {msg}");
        }
    } else {
        let backtraces: Vec<_> = err.backtrace.entries_iter().collect();
        report_from_msg_and_location(&msg, &backtraces);
    }
}

pub(crate) fn report_from_vm_exception(vm: &mut VirtualMachine, exc: &VmException) {
    let mut cur_frame = Frame::default();
    let msg = exc.value.prettyprint(&mut cur_frame, vm);
    let backtraces: Vec<_> = exc.backtrace.entries_iter().collect();
    report_from_msg_and_location(&msg, backtraces.as_slice());
}

pub(crate) fn report_from_compiler_error(err: &CompilationError) {
    let msg = err.reason.to_string();
    let loc = &err.loc;
    report_from_msg_and_location(&msg, &[loc]);
}

pub(crate) fn report_from_parser_error(err: &ParserError) {
    let msg = &err.msg;
    let loc = &err.loc;
    report_from_msg_and_location(msg, &[loc]);
}
