use criterion::{black_box, Criterion, criterion_group, criterion_main};

use rust_naml::lexer::naml_lexer::NamlLexer;
use rust_naml::parser::naml_parser::NamlParser;

fn read_file() -> Vec<u8> {
    let file = std::fs::read_to_string("./benches/resources/example.naml").expect("Failed to read example naml file");
    file.into_bytes()
}

fn lexer(c: &mut Criterion) {
    let source = read_file();

    let mut group = c.benchmark_group("lexer");
    group.sample_size(1000);
    group.bench_function("tokenize", |b| b.iter(|| NamlLexer::tokenize(black_box(&source))));
    group.finish();
}

fn parser(c: &mut Criterion) {
    let source = read_file();
    let mut tokens = NamlLexer::tokenize(&source).expect("Failed to lex example naml file");

    let mut group = c.benchmark_group("parser");
    group.sample_size(1000);
    group.bench_function("parse-token-stream", |b| b.iter(|| NamlParser::parse_tokens(black_box(&mut tokens))));
    group.bench_function("parse-and-tokenize", |b| b.iter(|| NamlParser::parse(black_box(&source))));
    group.finish();
}

criterion_group!(benches, parser, lexer);
criterion_main!(benches);
