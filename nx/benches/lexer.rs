use criterion::{black_box, criterion_group, criterion_main, Criterion};
//
use nx::lexer::lex;

fn lexer(c: &mut Criterion) {
    let code = "fn module struct ;:,.-/ +**# return let x+y var 99 instance use ()[]{} // comment";

    fn tokenize_string(code: &str) {
        let code = black_box(code);
        let _ = lex::tokenize_string(&code).unwrap();
    }

    c.bench_function("lexer_tokenize_string", |b| {
        b.iter(|| tokenize_string(&code))
    });
}

criterion_group!(benches, lexer);
criterion_main!(benches);
