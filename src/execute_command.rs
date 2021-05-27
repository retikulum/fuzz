//rand kütüphanesini ekliyoruz
use rand::Rng;
//std kütüphanesi diğer dillerdekiyle aynı
use std::io::{self, Write};
//komut çalıştırmak için process::Command'ı ekliyoruz
use std::process::Command;


fn main(){

    
    //rand kütüphanesinin gen fonksiyonuyla 32 tane u8 tipinde değer üretiyoruz
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
    println!("Yazılacak bytelar:");
    println!("{:?}", random_bytes);

    //dosya açıyoruz.https://stackoverflow.com/questions/53826371/how-to-create-a-binary-file-with-rust/53827079
    let mut file = std::fs::File::create("input").expect("create failed");

    //byte array'i dosyaya yazıyoruz.
    file.write_all(&random_bytes).expect("write failes");

    //File komutunu çalıştırıp oluşturduğumuz dosyayı argüman olarak veriyoruz.
    let output = Command::new("file").arg("input").output().expect("failed to execute");
    io::stdout().write_all(&output.stdout).unwrap();

}