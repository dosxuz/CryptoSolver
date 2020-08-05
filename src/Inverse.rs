pub fn GetMultiplicativeInverse(num : i32) -> i32 {
    let mut i : i32;
    let mut MI : i32=0;
    for i in 1..num {
        MI = ((i*26)+1);
        if MI%num == 0 {
            break;
        }
    }
    MI = MI/num;
    return MI;
}