use clap::Parser;

#[derive(Parser)]
struct CLI {
    #[arg(short = 'l', long = "lexx")]
    lexx: String,
}

#[tokio::main]
async fn main() {
    let response = match knightpark::get_response().await {
        Ok(v) => v,
        Err(_) => {
            println!("Error contacting Parking API.");
            return;
        }
    };

    response.into_iter().for_each(|element| {
        println!("{:?}", element.location.name);
    });
}
