use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
  let lib_name = "reloadable";

  let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let cargo_toml_path = Path::new(&cargo_manifest_dir).join("Cargo.toml");

  let mut cargo_toml_src = File::open(&cargo_toml_path).unwrap();
  let mut data = String::new();
  cargo_toml_src.read_to_string(&mut data).unwrap();
  drop(cargo_toml_src);

  let search = format!("name = \"{}", lib_name);
  let line = data.lines().find(|l| l.contains(&search));

  if let Some(line) = line {
    let epoch_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let replace = format!("name = \"{}_{}\"", lib_name, epoch_time.as_secs());

    let mut cargo_toml_dst = File::create(&cargo_toml_path).unwrap();
    cargo_toml_dst
      .write(data.replace(line, &replace).as_bytes())
      .unwrap();
  }
}
