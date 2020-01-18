mod calculator{
    pub mod to_cal{
        pub fn sum(x:u32, y:u32) -> u32{
            let a=x+y;
            a
        }
    }
}
use calculator::to_cal::sum;
fn main() {
    let x =10;
    let y =5;
    let result = sum(x,y);
    println!("Sum = {}",result);
}
