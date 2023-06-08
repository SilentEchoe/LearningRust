fn main() {
    let ip1 = IPAddrKind::V4;
    let ip_str = match ip1 {
        IPAddrKind::V4 => "V4",
        _ => "V6",
    };
    println!("ip1 is {}", ip_str);
}

enum IPAddrKind {
    V4,
    V6,
}
