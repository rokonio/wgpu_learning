use naga::front::wgsl::Parser;

fn main() {
    let mut parser = Parser::new();
    let shader = include_bytes!("shaders/shader.wgsl");
    parser
        .parse(std::str::from_utf8(&shader[..]).unwrap())
        .unwrap();
}
