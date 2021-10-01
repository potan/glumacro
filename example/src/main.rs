//#![feature(proc_macro_hygiene)]

fn main() {
    println!("{:?}", glumacro::a_proc_macro!(r#"
    let lim = 36
    let array = import! std.array
    let int = import! std.int    
    rec let erotosfen n p = if n > lim
                            then []
                            else let next = erotosfen (n+1) in if p n
                                                            then next p
                                                            else array.append [n] (next (\x -> p x || int.rem x n == 0))
    in show (erotosfen 2 (\x -> False))
    "#));
}
