use std::hint::black_box;

use aria_compiler::compile_from_source;
use aria_parser::ast::SourceBuffer;
use criterion::{Criterion, criterion_group, criterion_main};
use haxby_vm::
    haxby_eval
;

fn bench_if(c: &mut Criterion) {
    const INPUT: &str = r#"
    func main() {
        if true {

        } else {

        }
        if false {

        } else {

        }
    }
    "#;

    c.bench_function("control_flow/if/compile", |b| {
        b.iter(|| {
            let sb = SourceBuffer::stdin(INPUT);
            black_box(
                compile_from_source(&sb, &Default::default()).expect("module did not compile"),
            );
        })
    });

    let sb = SourceBuffer::stdin(INPUT);

    c.bench_function("control_flow/if/eval", |b| {
        b.iter_batched(
            || compile_from_source(&sb, &Default::default()).expect("module did not compile"),
            |module| black_box(haxby_eval(module, Default::default()).unwrap()),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_for(c: &mut Criterion) {
    const INPUT: &str = r#"
    func main() {
        for i in Range.from(0).to(10) {
            println(i);
        }
    }
    "#;

    c.bench_function("control_flow/for/compile", |b| {
        b.iter(|| {
            let sb = SourceBuffer::stdin(INPUT);
            black_box(
                compile_from_source(&sb, &Default::default()).expect("module did not compile"),
            );
        })
    });

    let sb = SourceBuffer::stdin(INPUT);

    c.bench_function("control_flow/for/eval", |b| {
        b.iter_batched(
            || compile_from_source(&sb, &Default::default()).expect("module did not compile"),
            |module| black_box(haxby_eval(module, Default::default()).unwrap()),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_while(c: &mut Criterion) {
    const INPUT: &str = r#"
    func main() {
        val i = 0;
        while i < 10 {
            i = i + 1;
        }
    }
    "#;

    c.bench_function("control_flow/while/compile", |b| {
        b.iter(|| {
            let sb = SourceBuffer::stdin(INPUT);
            black_box(
                compile_from_source(&sb, &Default::default()).expect("module did not compile"),
            );
        })
    });

    let sb = SourceBuffer::stdin(INPUT);

    c.bench_function("control_flow/while/eval", |b| {
        b.iter_batched(
            || compile_from_source(&sb, &Default::default()).expect("module did not compile"),
            |module| black_box(haxby_eval(module, Default::default()).unwrap()),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(control_flow, bench_if, bench_for, bench_while);
criterion_main!(control_flow);
