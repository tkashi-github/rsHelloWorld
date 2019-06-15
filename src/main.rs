fn main() {
    let mut cnt: u32 = 0;
    //let mut _i:u32 = 0;
    println!("Hello, world!");

    for _i in 0..100{
        if 0 == (cnt % 2) {
            println!("even {}", cnt);
        }
        else
        {
            println!("odd {}", cnt);
        }
        cnt += 1;
    }
}
