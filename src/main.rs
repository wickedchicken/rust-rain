use rust_rain;
use structopt::StructOpt;

fn main() {
    let opts = rust_rain::Opt::from_args();
    rust_rain::draw_rain(&opts);
}
