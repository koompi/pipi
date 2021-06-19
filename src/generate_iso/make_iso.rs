pub use run_script::{run_script, ScriptOptions};

pub fn iso() {
    println!("Making KOOMPI's iso...");

    let (code, output, error) = run_script::run_script!(
            r#"
        source config
	    rm -f $OUTPUT
	    xorriso -as mkisofs \
	    	  -isohybrid-mbr "$FILESDIR"/isohdpfx.bin \
	    	-c isolinux/boot.cat \
	    	-b isolinux/isolinux.bin \
	    	  -no-emul-boot \
	    	  -boot-load-size 4 \
	    	  -boot-info-table \
	    	  -eltorito-alt-boot \
	    	-e boot/efiboot.img \
	    	  -no-emul-boot \
	    	  -isohybrid-gpt-basdat \
	    	  -volid $ISOLABEL \
	    	-o $OUTPUT $LIVEWDIR
        "#
        )
        .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
