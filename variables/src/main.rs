trait IpAddr {
    fn display(&self);
}

struct Ipv4(String);
impl IpAddr for Ipv4 {
    fn display(&self) {
        println!("Ipv4: {}", self.0);
    }
}

struct Ipv6(String);
impl IpAddr for crate::Ipv6 {
    fn display(&self) {
        println!("Ipv6: {}", self.0);
    }
}


fn main() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(Ipv4(String::from("127.0.0.1".to_string()))),
        Box::new(Ipv6(String::from("::1".to_string())))];

    for ip in v {
        ip.display();
    }
}
