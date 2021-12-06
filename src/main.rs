mod ex1;
mod ex2;
mod ex3;
use ex1::*;
use ex2::*;
use ex3::data_capture::*;

fn main() {
    let ex1 = Ex1::new(55);
    println!("length: {}", ex1.len());
    println!("average: {:.1$}", ex1.mean(), 3);
    println!("median: {}", ex1.median());
    println!("mode: {}", ex1.mode());

    let nonplatin = " some unnecessary verbiage will need to be elided, but otherwise the article is publishable";
    let ex2 = PigLatin::new(String::from(nonplatin));
    println!("non plat: {}", nonplatin);
    println!("platinised: {}", ex2.translate());

    let mut dcapt = DataAggregator::new();
    dcapt.input_loop();
}
