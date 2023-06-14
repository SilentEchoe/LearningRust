fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // 使用 `matches` 填空
    for ab in alphabets {
        assert!(matches!(ab,'A'..='Z' | 'a'..='z'))
    }
}
