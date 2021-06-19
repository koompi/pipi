pub use run_script::{run_script, ScriptOptions};

pub fn efiboot() {
    println!("Setup efitboot...");

    let (code, output, error) = run_script::run_script!(
        r#"
    source config
    mkdir -p "$LIVEWDIR"/boot/{grub/{fonts,x86_64-efi},EFI}

    if [ -f /usr/share/grub/unicode.pf2 ];then
    	cp /usr/share/grub/unicode.pf2 "$LIVEWDIR"/boot/grub/fonts
    fi
    if [ -f "$FILESDIR"/isolinux/splash.png ]; then
    	cp "$FILESDIR"/isolinux/splash.png "$LIVEWDIR"/boot/grub/
    fi

    echo "set prefix=/boot/grub" > "$LIVEWDIR"/boot/grub-early.cfg
    cp -a /usr/lib/grub/x86_64-efi/*.{mod,lst} "$LIVEWDIR"/boot/grub/x86_64-efi
    cp "$FILESDIR"/grub.cfg "$LIVEWDIR"/boot/grub/

    grub-mkimage \
    	-c "$LIVEWDIR"/boot/grub-early.cfg \
    	-o "$LIVEWDIR"/boot/EFI/bootx64.efi \
    	-O x86_64-efi \
    	-p "" iso9660 normal search search_fs_file

    modprobe loop
    dd if=/dev/zero of="$LIVEWDIR"/boot/efiboot.img count=4096
    mkdosfs -n KOOMPILIVE "$LIVEWDIR"/boot/efiboot.img
    mkdir -p "$EFIBOOTDIR"
    mount -o loop "$LIVEWDIR"/boot/efiboot.img "$EFIBOOTDIR"
    mkdir -p "$EFIBOOTDIR"/EFI/boot
    cp "$LIVEWDIR"/boot/EFI/bootx64.efi "$EFIBOOTDIR"/EFI/boot

    unmount "$EFIBOOTDIR"
    rm -fr "$EFIBOOTDIR"
        "#
        )
        .unwrap();

        println!("Exit Code: {}", code);
        println!("Output: {}", output);
        println!("Error: {}", error);
}
