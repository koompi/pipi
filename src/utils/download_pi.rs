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
    } else {
        let (code, output, error) = run_script::run_script!(
            r#"
         cd pi
         cargo build --release --bin pi
         echo "Finished Compile Pi"
         cargo build --release --bin bin-repo
         echo "Finished Compile bin-repo"
         cargo build --release --bin server
         echo "Finished Compile server"
         cargo build --release --bin source-repo
         echo "Finished Compile source-repo"
         sudo install -Dm755 target/release/pi /usr/bin/pi
         sudo install -Dm755 target/release/bin-repo /usr/bin/bin-repo
         sudo install -Dm755 target/release/server /usr/bin/server
         sudo install -Dm755 target/release/source-repo /usr/bin/source-repo
         sudo cp files/xchroot /bin
         "#
        )
        .unwrap();

        println!("Exit Code: {}", code);
        println!("Output: {}", output);
        println!("Error: {}", error);
    }

    let (code, output, error) = run_script::run_script!(
        r#"
         cd pi
         cargo build --release --bin pi
         echo "Finished Compile Pi"
         cargo build --release --bin bin-repo
         echo "Finished Compile bin-repo"
         cargo build --release --bin server
         echo "Finished Compile server"
         cargo build --release --bin source-repo
         echo "Finished Compile source-repo"
         sudo install -Dm755 target/release/pi /usr/bin/pi
         sudo install -Dm755 target/release/bin-repo /usr/bin/bin-repo
         sudo install -Dm755 target/release/server /usr/bin/server
         sudo install -Dm755 target/release/source-repo /usr/bin/source-repo
         sudo cp files/xchroot /bin
         sudo cp -r ../../pipi-live /opt/
         "#
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
