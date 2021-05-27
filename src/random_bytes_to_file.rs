//rand kütüphanesini ekliyoruz
use rand::Rng;

//std kütüphanesi diğer dillerdekiyle aynı

use std::io::Write;

fn main(){

    //rand kütüphanesinin gen fonksiyonuyla 32 tane u8 tipinde değer üretiyoruz
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
    println!("Yazılacak bytelar:");
    println!("{:?}", random_bytes);
    //dosya açıyoruz.https://stackoverflow.com/questions/53826371/how-to-create-a-binary-file-with-rust/53827079
    let mut file = std::fs::File::create("input").expect("create failed");
    //byte array'i dosyaya yazıyoruz.
    file.write_all(&random_bytes).expect("write failes");
    
}