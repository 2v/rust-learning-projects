struct Hostname(String); // Hostname wraps String using the newtype pattern

fn connect(host: Hostname) {
    println!("connected to {}", host.0)
}

fn main() {
    let ordinary_string = String::from("just an ordinary string");
    let host = Hostname( ordinary_string.clone() );

    connect(ordinary_string);
}