use ks;

fn main() -> Result<(), Box<dyn std::error::Error>> {

  ks::document::format("Documents")?;

  Ok(())
}
