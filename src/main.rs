mod server;

fn main() {
    let listener = server::listen();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        server::connect(stream);
    }
}