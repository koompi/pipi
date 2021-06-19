pub use run_script::{run_script, ScriptOptions};

pub fn efiboot() {
    println!("Setup efitboot...");

    let (code, output, error) = run_script::run_script!(
        r#"
    mkdir -p work/live/boot/grub/fonts
    mkdir -p work/live/boot/grub/x86_64-efi
    mkdir -p work/live/boot/EFI

    if [ -f /usr/share/grub/unicode.pf2 ];then
    	cp /usr/share/grub/unicode.pf2 work/live/boot/grub/fonts
    fi
    if [ -f files/isolinux/splash.png ]; then
    	cp files/isolinux/splash.png work/live/boot/grub/
    fi

    echo "set prefix=/boot/grub" > work/live/boot/grub-early.cfg
    cp -a /usr/lib/grub/x86_64-efi/*.mod work/live/boot/grub/x86_64-efi
    cp -a /usr/lib/grub/x86_64-efi/*.lst work/live/boot/grub/x86_64-efi
    cp files/grub.cfg work/live/boot/grub/

    grub-mkimage \
    	-c work/live/boot/grub-early.cfg \
    	-o work/live/boot/EFI/bootx64.efi \
    	-O x86_64-efi \
    	-p "" iso9660 normal search search_fs_file

    modprobe loop
    dd if=/dev/zero of=work/live/boot/efiboot.img count=4096
    mkdosfs -n KOOMPILIVE work/live/boot/efiboot.img
    mkdir -p work/efiboot
    mount -o loop work/live/boot/efiboot.img work/live/efiboot
    mkdir -p work/live/efiboot/EFI/boot
    cp work/live/boot/EFI/bootx64.efi work/live/efiboot/EFI/boot

    unmount work/live/efiboot
    rm -fr work/live/efiboot
        "#
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
