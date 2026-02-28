use std::{env, path::{self, Path}};

pub fn format(dir_name: &str) -> Result<(), Box<dyn std::error::Error>> {
  // Get home dir.
  let mut desired_dir = env::home_dir().ok_or("Home dir does not exist!")?;
  
  // append desired dir.
  desired_dir.push(dir_name);


  // Check if dir exists
    let path_exists = desired_dir.try_exists()? && desired_dir.is_dir();


  println!("{}", desired_dir.display());
  println!("{0} exists? {1}", desired_dir.display(), path_exists);

  Ok(())
}