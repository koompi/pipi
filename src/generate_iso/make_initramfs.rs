pub use run_script::{run_script, ScriptOptions};

pub fn initramfs() {
    println!("Preparing initramfs....");

    let (code, output, error) = run_script::run_script!(
        r#"
    source config
    chroot_run() {
        xchroot rootfs $@
        return $?

    }
	sed "s/@ISOLABEL@/KOOMPILIVE/g" files/livecd.hook > rootfs/usr/share/mkinitramfs/hooks/livecd.hook
	cp rootfs/boot/vmlinuz-koompi work/live/boot/vmlinuz
	kernver=$(file rootfs/boot/vmlinuz-koompi | cut -d ' ' -f9)
	chroot_run mkinitramfs -k $kernver -a livecd -o /boot/initrd-koompi.img
	mv rootfs/boot/initrd-koompi.img work/live/boot/initrd

    "#
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
