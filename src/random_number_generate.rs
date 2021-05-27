use rand::Rng;

fn main(){
    let mut rng = rand:Ç:thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Rastgele u8 {}" , n1);
    println!("Rastgele u16 {}" , n2);
}