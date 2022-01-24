#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(
        name: &str,
        data: &Vec<u8>
    ) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f 
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &mut File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f2 = File::new_with_data("2.txt", &data);

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&mut f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer); // converts Vec<u8> to String. Any bytes that are not valid UTF-8 are replaced with "?"

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);
}