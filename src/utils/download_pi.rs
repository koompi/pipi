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
         env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin pi
         env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin bin-repo
         env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin server
         env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin source-repo

         sudo install -Dm755 target/release/pi /usr/bin/pi
         sudo install -Dm755 target/release/bin-repo /usr/bin/bin-repo
         sudo install -Dm755 target/release/server /usr/bin/server
         sudo install -Dm755 target/release/source-repo /usr/bin/source-repo
         sudo install -Dm755 files/xchroot /bin
         sudo git clone https:/github.com/koompi/pipi-live /opt/
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
         env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin pi
         env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin bin-repo
         env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin server
         env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin source-repo

         sudo install -Dm755 target/release/pi /usr/bin/pi
         sudo install -Dm755 target/release/bin-repo /usr/bin/bin-repo
         sudo install -Dm755 target/release/server /usr/bin/server
         sudo install -Dm755 target/release/source-repo /usr/bin/source-repo
         sudo install -Dm755 files/xchroot /bin
         sudo git clone https:/github.com/koompi/pipi-live /opt/
         "#
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
