
fn main() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
    let v = vec!(1, 2, 3);
    is_vec(&v);

    // in code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE `for` to rewrite the below code
    let mut v1 = Vec::new();
    for i in &v {
        v1.push(*i)
    }
    is_vec(&v1);

    assert_eq!(format!("{:?}",v), format!("{:?}",v1));


    println!("Success!")
}

fn is_vec(v: &Vec<u8>) {

}