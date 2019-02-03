#[macro_use]
extern crate log;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug)]
pub struct App {
    args: Args,
}

impl App {
    pub fn init() -> App {
        let args = Args::from_args();

        env_logger::Builder::new()
            .filter_level(args.verbose.log_level().to_level_filter())
            .init();

        debug!("CLI args: {:?}", args);

        App { args }
    }

    pub fn args(&self) -> &Args {
        &self.args
    }

    pub fn run(&self) -> Result<(), failure::Error> {
        Ok(println!("Hello World!"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("it works");
    }
}
