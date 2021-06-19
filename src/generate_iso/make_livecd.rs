use std::process::Command;
pub use crate::generate_iso::{cleaning::cleaning, make_efiboot::efiboot, make_initramfs::initramfs, make_iso::iso, make_liveworkdir::liveworkdir, make_squashfs::squashfs};
pub use crate::{generate_iso::{make_efiboot, make_initramfs, make_iso, make_liveworkdir, make_squashfs}};

pub fn livecd() {
    println!("Building livecd....");

    Command::new("set")
            .arg("-e")
            .spawn()
            .expect("set -e failed to execute");


    liveworkdir();
    initramfs();
    squashfs();
    efiboot();
    iso();
    cleaning();

    Command::new("set")
    .arg("-e")
    .spawn()
    .expect("set -e failed to execute");
}
