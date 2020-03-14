use std::fs;
use std::io::{Write, Read, BufWriter, BufReader, copy};

fn file_io() {
    {
        // write
        let string = "Hello, file io!";
        let mut f = fs::File::create("test.txt").unwrap(); // open file, you can write to file.
        // "create" open as write only mode.
        f.write_all(string.as_bytes()).unwrap(); // byte-only

        // file is closed here.
    }

    {
        // read
        let mut f = fs::File::open("test.txt").unwrap();
        // "open" open as read only mode
        let mut buf = vec![];

        f.read_to_end(&mut buf).unwrap(); // read here
        println!("{}", std::str::from_utf8(&buf).unwrap()); // to &str
        // println!("{}", String::from_utf8(b).unwrap()); // or String
    }

    // seek
    // f.seek(std::io::SeekFrom::Start(3)) // seek points head + 3 byte.
}

fn buffer_io() {
    {
        // write
        let b = "Hello, buffer io!";
        let mut f = BufWriter::new(fs::File::create("test.txt").unwrap()); // buffering write
        f.write(b.as_bytes()).unwrap();
    }


    {
        // read
        let mut f = BufReader::new(fs::File::open("test.txt").unwrap()); // buffering read
        let mut buf = vec![];

        f.read_to_end(&mut buf).unwrap();

        println!("{}", std::str::from_utf8(&buf).unwrap());
    }
}

fn string_io() {
    let mut buf = vec![];
    {
        // write
        let test = "Hello, string io!";
        let mut f = BufWriter::new(&mut buf);
        f.write(test.as_bytes()).unwrap();
    }
    let string = std::str::from_utf8(&buf).unwrap();
    println!("{}", string);

    {
        // read
        let mut buf = BufReader::new(string.as_bytes());
        let mut read_str = String::new();

        buf.read_to_string(&mut read_str).unwrap();

        println!("{}", read_str);
    }

}

#[allow(dead_code)]
fn net_io() {
    use std::net;

    {
        // connect
        let s = "Hello, network io!";
        let mut stream = net::TcpStream::connect("127.0.0.1:30001").unwrap();
        let _ = stream.write(s.as_bytes());

        {
            let mut buf = String::new();
            stream.read_to_string(&mut buf).unwrap(); // read from stream, until EOF
            println!("{}", buf);
        }
    }

    {
        // listen
        let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();
        match listener.accept() {
            Ok((_socket, _addr)) => println!("Connection comes!"),
            Err(_e) => println!("Error"),
        }
    }
}

fn copy_io() {
    let mut writer = vec![];
    let s = "Hello, copy reader to writer!";
    let mut reader = BufReader::new(s.as_bytes());

    copy(&mut reader, &mut writer).unwrap();

    println!("{}", std::str::from_utf8(&writer).unwrap());


}

fn main() {
    // if you writes much bytes, you should buffering

    file_io();
  //  buffer_io();
  //  string_io();
    // net_io();
  //  copy_io();

    // string add
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
}