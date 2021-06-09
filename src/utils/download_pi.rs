pub use cmd_lib::run_cmd;
pub use git2::Repository;

pub fn download_pi() {
    // let url = "https://github.com/koompi/pi";
    // let repo = match Repository::clone(url, "pi") {
    //     Ok(repo) => repo,
    //     Err(e) => panic!("failed to clone: {}", e),
    // };

    run_cmd! (
        info "Dropping caches at first";
        cd pi;
        pwd;
        cargo build --release --bin pi;
        echo "Finished Compile Pi";
        cargo build --release --bin bin-repo;
        echo "Finished Compile bin-repo";
        cargo build --release --bin server;
        echo "Finished Compile server";
        cargo build --release --bin source-repo;
        echo "Finished Compile source-repo";
        info "It workking";
    )
    .unwrap();
}

// run_cmd! (
//     info "Dropping caches at first";
//     cd pi;
//     env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin pi;
//     echo "Finished Compile Pi";
//     env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin bin-repo;
//     echo "Finished Compile bin-repo";
//     env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin server;
//     echo "Finished Compile server";
//     env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin source-repo;
//     echo "Finished Compile source-repo";
//     info "It workking";
// ).unwrap();
