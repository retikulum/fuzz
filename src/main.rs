//rand kütüphanesini ekliyoruz
use rand::Rng;

//std kütüphanesi diğer dillerdekiyle aynı

use std::io::Write;

fn main(){

    //rand kütüphanesinin gen fonksiyonuyla 32 tane u8 tipinde değer üretiyoruz
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
    println!("{:?}", random_bytes);    
}