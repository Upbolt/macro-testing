use macro_derive::HasBar;
use macro_testing::WithBar;

#[derive(HasBar)]
struct Foo {
    fizz: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", Foo::bar());

    Ok(())
}
