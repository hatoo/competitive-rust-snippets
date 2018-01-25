use std;

// println! is very slow when print more than 10^5 lines.
#[snippet = "with_bufwriter"]
#[allow(dead_code)]
fn with_bufwriter<F: FnOnce(std::io::BufWriter<std::io::StdoutLock>) -> ()>(f: F) {
    let out = std::io::stdout();
    let writer = std::io::BufWriter::new(out.lock());
    f(writer)
}

#[test]
fn test_with_bufwriter() {
    // sample code
    use std::io::Write;
    with_bufwriter(|mut out| {
        writeln!(out, "ok").unwrap();
    });
}
