pub fn calc_gcd(num : i32)-> i32 {
    let mut x:i32;
    let mut temp1:i32 = num;
    let mut temp2:i32=26;

    while temp2 != 0 {
        x = temp2;
        temp2 = temp1%temp2;
        temp1=  x;
    }
    return temp1;

}