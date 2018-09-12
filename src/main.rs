extern crate toml;

fn main() -> Result<(), String> {
  let action = std::env::args().nth(1).ok_or("Missing first parameter")?;
  match action.as_ref() {
    "sh" => get_aliases(),
    _ => Err("Invalid first parameter".to_string()),
  }
}

fn get_aliases() -> Result<(), String> {
  let config_file = std::env::home_dir()
    .ok_or("Couldn't get home directory")?
    .join(".flatpak-aliases");

  let config: toml::value::Table = toml::from_str(
    std::fs::read_to_string(config_file)
      .map_err(|_| "Could not read config file")?
      .as_str(),
  ).map_err(|_| "Invalid toml in config file")?;

  for (key, mut val) in config {
    let args = build_flatpak_args(val.as_table_mut().unwrap())?;
    print!("alias \"{}\"=\"flatpak run {}\";", key, args);
  }

  Ok(())
}

fn build_flatpak_args(args: &mut toml::value::Table) -> Result<String, String> {
  let app = args.get("app").ok_or("Missing app value")?.to_string();
  args.remove("app");

  Ok(
    args
      .iter()
      .map(|(key, val)| match val.type_str() {
        "string" => format!("--\"{}\"=\"{}\"", key, val),
        "array" => val
          .as_array()
          .unwrap()
          .into_iter()
          .map(|val| format!("--\"{}\"=\"{}\"", key, val))
          .fold("".to_string(), |accum, s| accum + &s + " "),
        "boolean" => if val.as_bool().unwrap() {
          format!("--{}", key)
        } else {
          "".to_string()
        },
        _ => panic!("Invalid type for {}", key),
      })
      .fold("".to_string(), |accum, s| accum + &s + " ") + &app,
  )
}
