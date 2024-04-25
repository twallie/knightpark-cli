use clap::Parser;
use knightpark::garages::{Garage, Garages};

#[derive(Parser)]
struct CLI {
    #[arg(long = "garage-a")]
    show_a: bool,
    #[arg(long = "garage-b")]
    show_b: bool,
    #[arg(long = "garage-c")]
    show_c: bool,
    #[arg(long = "garage-d")]
    show_d: bool,
    #[arg(long = "garage-h")]
    show_h: bool,
    #[arg(long = "garage-i")]
    show_i: bool,
    #[arg(long = "garage-j")]
    show_j: bool,
}

#[tokio::main]
async fn main() {
    let args = CLI::parse();
    let show_all = 
        !args.show_a &&
        !args.show_b &&
        !args.show_c &&
        !args.show_d &&
        !args.show_h &&
        !args.show_i &&
        !args.show_j;

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
    let mut vec: Vec<Option<&Garage>> = vec![];
    if show_all || args.show_a {
        vec.push(garages.a.as_ref());
    }
    if show_all || args.show_b {
        vec.push(garages.b.as_ref());
    }
    if show_all || args.show_c {
        vec.push(garages.c.as_ref());
    }
    if show_all || args.show_d {
        vec.push(garages.d.as_ref());
    }
    if show_all || args.show_h {
        vec.push(garages.h.as_ref());
    }
    if show_all || args.show_i {
        vec.push(garages.i.as_ref());
    }

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
