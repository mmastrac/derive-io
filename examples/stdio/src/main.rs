use derive_io::{AsFileDescriptor, Read, Write};

#[derive(Read, Write, AsFileDescriptor)]
struct StdioStreams {
    #[read]
    #[descriptor]
    stdin: std::io::Stdin,
    #[write]
    stdout: std::io::Stdout,
}

#[derive(Read, AsFileDescriptor)]
enum Generic<S> {
    Generic(
        #[read]
        #[descriptor]
        S,
    ),
    File(
        #[read]
        #[descriptor]
        std::fs::File,
    ),
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
