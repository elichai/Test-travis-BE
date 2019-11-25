
#[cfg(target_endian = "little")]
fn print() {
    println!("little");
}

#[cfg(target_endian = "big")]
fn print() {
    println!("big");
}


#[cfg(all(not(target_endian = "big"), not(target_endian = "little")))]
fn print() {
    println!("shit");
}

fn main() {
    print();
}
