pub use cmd_lib::run_cmd;
pub use git2::Repository;
pub use run_script::{run_script, ScriptOptions};
use std::path::Path;

pub fn download_pi() {
    let pi_exist = Path::new("pi").exists();

    if pi_exist == false {
        let url = "https://github.com/koompi/pi";
        let repo = match Repository::clone(url, "pi") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
    }

    let (code, output, error) = run_script::run_script!(
        r#"
         cd pi
         cargo build --release --bin pi
         echo "Finished Compile Pi"
         cargo build --release --bin bin-repo;
         echo "Finished Compile bin-repo"
         cargo build --release --bin server
         echo "Finished Compile server"
         cargo build --release --bin source-re
         echo "Finished Compile source-repo"
         install -Dm755 target/release/pi /usr/bin/pi
         install -Dm755 target/release/bin-repo /usr/bin/bin-repo
         install -Dm755 target/release/server /usr/bin/server
         install -Dm755 target/release/source-repo /usr/bin/source-repo
         cp files/xchroot /bin
         "#
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
