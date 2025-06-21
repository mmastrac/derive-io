use std::io::{BufRead, BufReader};

use derive_io::{AsFileDescriptor, BufRead, Read, Write};

/// [`StdioStreams`] - Tests structs with separate read and write stream halves.
#[derive(Read, Write, AsFileDescriptor)]
struct StdioStreams {
    #[read]
    #[descriptor]
    stdin: std::io::Stdin,
    #[write]
    stdout: std::io::Stdout,
}

/// [`Generic`] - Tests generic structs with read-only functionality.
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

#[derive(Read, BufRead, AsFileDescriptor)]
struct GenericBufRead<S>(
    #[read]
    #[descriptor]
    S,
);

pub fn run() {
    use std::io::{Read, Write};

    let mut streams = StdioStreams {
        stdin: std::io::stdin(),
        stdout: std::io::stdout(),
    };

    let mut buf = [0; 1];
    _ = streams.write(&buf).unwrap();

    let mut file: Generic<std::fs::File> =
        Generic::File(std::fs::File::open("Cargo.toml").unwrap());
    _ = file.read(&mut buf).unwrap();

    let mut file: Generic<std::fs::File> =
        Generic::Generic(std::fs::File::open("Cargo.toml").unwrap());
    _ = file.read(&mut buf).unwrap();

    let mut s = String::new();
    let mut file = GenericBufRead(BufReader::new(std::fs::File::open("Cargo.toml").unwrap()));
    _ = file.read_line(&mut s).unwrap();
}
