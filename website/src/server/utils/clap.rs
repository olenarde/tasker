use clap::Parser;

#[derive(Parser, Clone)]
pub struct Opts {
    #[clap(long, env)]
    pub conn: String,
}
