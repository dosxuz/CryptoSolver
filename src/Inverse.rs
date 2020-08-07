pub fn GetMultiplicativeInverse(num : i32) -> i32 {
    let mut mi : i32=0;
    for i in 1..num {
        mi = (i*26)+1;
        if mi%num == 0 {
            break;
        }
    }
    mi = mi/num;
    return mi;
}
