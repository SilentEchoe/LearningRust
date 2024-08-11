
// 修复错误
fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // 越界访问将导致 panic.
    // 修改时必须使用 `v.len`
    let slice2 = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    // // 切片是只读的
    // // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);

    let slice3 = &mut v[0..4];


    assert_eq!(slice3, &[1, 2, 3, 4]);
    println!("Success!")
}