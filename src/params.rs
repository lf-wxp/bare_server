use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Params {
  #[clap(value_parser, default_value = "")]
  #[arg(short, long)]
  pub backend: String,

  #[clap(value_parser, default_value = "")]
  #[arg(short, long)]
  pub pass_id: String,
}
