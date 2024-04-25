use clap::Parser;
use knightpark::garages::{Garage, Garages};

#[derive(Parser)]
struct CLI {
    #[arg(short = 'l', long = "lexx")]
    lexx: String,
}

#[tokio::main]
async fn main() {
    let mut garages = Garages::new();
    match garages.refresh().await {
        Ok(_) => (),
        Err(_) => {
            println!("Error!");
            return;
        },
    }
    
    let mut garages = Garages::new();
    let _ = garages.refresh().await;

    // Putting properties into vector to process
    let vec: Vec<Option<&Garage>> = vec![
        garages.a.as_ref(),
        garages.b.as_ref(),
        garages.c.as_ref(),
        garages.d.as_ref(),
        garages.h.as_ref(),
        garages.i.as_ref()
    ];

    for option in vec {
        let garage = match option {
            Some(v) => v,
            None => {
                println!("NO GARAGE DATA");
                return;
            }
        };
        let available_f = garage.available as f32;
        let total_f = garage.total as f32;
        let percent: f32 = 100.0-((available_f/total_f)*100.0);
        println!("{:?}\tAvailable: {:?}\t\tTotal: {:?}\t% Full: {:.1}", garage.name, garage.available, garage.total, percent);
    }

}
