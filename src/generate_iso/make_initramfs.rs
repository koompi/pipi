pub use run_script::{run_script, ScriptOptions};

pub fn initramfs() {
    println!("Preparing initramfs....");

    let (code, output, error) = run_script::run_script!(
        r#"
    source config
    chroot_run() {
        xchroot $ROOTFS $@
        return $?

    }
	sed "s/@ISOLABEL@/$ISOLABEL/g" "$FILESDIR"/livecd.hook > $ROOTFS/usr/share/mkinitramfs/hooks/livecd.hook
	cp $ROOTFS/boot/vmlinuz-koompi "$LIVEWDIR"/boot/vmlinuz
	kernver=$(file $ROOTFS/boot/vmlinuz-koompi | cut -d ' ' -f9)
	chroot_run mkinitramfs -k $kernver -a livecd -o /boot/initrd-koompi.img
	mv $ROOTFS/boot/initrd-koompi.img "$LIVEWDIR"/boot/initrd

    "#
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
