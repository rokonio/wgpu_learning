use naga::front::wgsl::Parser;

fn main() {
    let mut parser = Parser::new();
    parser
        .parse(include_str!("src/shaders/shader.wgsl"))
        .unwrap();
}
