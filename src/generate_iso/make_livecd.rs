pub use nix::unistd::Uid;
pub use crate::generate_iso::{
    cleaning::cleaning, make_efiboot::efiboot, make_initramfs::initramfs, make_iso::iso,
    make_liveworkdir::liveworkdir, make_squashfs::squashfs,
};
pub use crate::generate_iso::{
    make_efiboot, make_initramfs, make_iso, make_liveworkdir, make_squashfs,
};
use std::process::Command;

pub fn livecd() {
    if !Uid::effective().is_root() {
        panic!("You need sudo permission to run use this feature");
    }
    println!("Building livecd....");

    liveworkdir();
    initramfs();
    squashfs();
    efiboot();
    iso();
    cleaning();
}
