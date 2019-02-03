fn main() -> Result<(), exitfailure::ExitFailure> {
    lfa::App::init().run().map_err(From::from)
}