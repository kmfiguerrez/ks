use std::{env, fs, io::Error, path::{Path, PathBuf}, process::Command};
use thiserror::Error;

pub mod document;

#[derive(Error, Debug)]
pub enum MyAppError {
    #[error("dir not found ")]
    NotFound,
    
    #[error("Network failure: {0}")]
    Disconnect(String),
}

pub fn pshell() {
  let status = Command::new("powershell")
    .args(["-Command", "get-alias"])
    .status()
    .expect("fuck nigga");
  println!("process finished with: {status}");
}

pub fn list_drives() -> Result<(), Box<dyn std::error::Error>> {
  let drives = [
    "A:\\", "B:\\", "D:\\", "E:\\", "F:\\", "G:\\", "H:\\", "I:\\",
    "J:\\", "K:\\", "L:\\", "M:\\", "N:\\", "O:\\", "P:\\", "Q:\\",
    "R:\\", "S:\\", "T:\\", "U:\\", "V:\\", "W:\\", "X:\\", "Y:\\",
    "Z:\\"
  ];

  for drive in drives {
    let path = Path::new(drive);
    // println!("is {} dir? {}", drive, path.is_dir());
    let exists = path.try_exists()?;
    // Get a the letter only.
    let char = match drive.chars().nth(0) {
      Some(dl) => dl,
      _ => return Ok(())
    };
    println!("{} drive does exists? {}", char, exists && path.is_dir());
    // format_partition(char);
  }

  Ok(())
}



pub fn format_partition(drive_letter: char) {
  let ps_script = format!("Get-Volume -DriveLetter {} | Format-Volume -FileSystem NTFS -Full:$false -Force", drive_letter);
  let status = Command::new("powershell")
    .args(["-Command", &ps_script])
    .status()
    .expect("fuck nigga no format");
}

pub fn tamrof_drive() -> std::io::Result<()> {
  let drives = [
    "A:\\", "B:\\", "D:\\", "E:\\", "F:\\", "G:\\", "H:\\", "I:\\",
    "J:\\", "K:\\", "L:\\", "M:\\", "N:\\", "O:\\", "P:\\", "Q:\\",
    "R:\\", "S:\\", "T:\\", "U:\\", "V:\\", "W:\\", "X:\\", "Y:\\",
    "Z:\\"
  ];

  for drive in drives {
    let path = Path::new(drive);
    // println!("is {} dir? {}", drive, path.is_dir());
    let exists = path.try_exists()?;
    // Get a the letter only.
    let char = match drive.chars().nth(0) {
      Some(dl) => dl,
      _ => return Ok(())
    };
    println!("{} drive does exists? {}", char, exists && path.is_dir());
    // format_partition(char);
  }

  Ok(())
}

pub fn get_cdir() -> Result<PathBuf, Error> {
  let current_dir = env::current_dir()?;
  Ok(current_dir)
}

pub fn create_dir(dir_name: &str) -> std::io::Result<()> {
  let path = format!("e:\\{}", dir_name);
  fs::create_dir(path)?;
  Ok(())
}

pub fn remove_dir(dir_name: &str) -> std::io::Result<()> {
  // let path = format!("e:\\{}", dir_name);
  let path = Path::new(dir_name);

  // Check first if dir exists
  let exists = path.try_exists()?;
  if exists && path.is_dir() {
    println!("{} exists!", dir_name);
    fs::remove_dir_all(path)?
  }

  Ok(())
}

pub fn make_dir(num: u8) -> std::io::Result<()> {
  // Check first if a drive exists.
  let path = Path::new("t:\\");
  let exists = path.try_exists()?;
  println!("{} exists?: {}", path.display(), exists);

  // for i in 1..5 {
  //   println!("{}", i);
  // }

  env::set_current_dir(path)?;
  let current_dir = env::current_dir()?;
  let mut counter: u8 = 1;
  while counter < (num + 1) {
    let path = format!("{}testfolder{}", current_dir.display(), counter);
    let path = Path::new(&path);
    // println!("{}. {}", counter, path.display());
    fs::create_dir(path)?;
    counter += 1;
  }

  Ok(())
}