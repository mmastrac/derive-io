use derive_io::{AsDescriptor, Read, Write};

#[derive(Read, Write, AsDescriptor)]
struct StdioStreams {
    #[read]
    #[descriptor]
    stdin: std::io::Stdin,
    #[write]
    stdout: std::io::Stdout,
}

fn main() {
    use std::io::{Read, Write};

    let mut streams = StdioStreams {
        stdin: std::io::stdin(),
        stdout: std::io::stdout(),
    };

    let mut buf = [0; 1];
    streams.read(&mut buf).unwrap();
    streams.write(&buf).unwrap();
}
