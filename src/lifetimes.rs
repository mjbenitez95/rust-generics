pub fn good_lifetime() {
    {
        let x = 5;
        let r = &x;
        println!("r: {}", r);
    }
}
