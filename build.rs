const COMMANDS: &[&str] = &["do_recaptcha_challenge"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
