pub use run_script::{run_script, ScriptOptions};

pub fn iso() {
    println!("Making KOOMPI's iso...");

    let (code, output, error) = run_script::run_script!(
        r#"
	    rm -f koompilive.iso
	    xorriso -as mkisofs \
	    	  -isohybrid-mbr files/isohdpfx.bin \
	    	-c isolinux/boot.cat \
	    	-b isolinux/isolinux.bin \
	    	  -no-emul-boot \
	    	  -boot-load-size 4 \
	    	  -boot-info-table \
	    	  -eltorito-alt-boot \
	    	-e boot/efiboot.img \
	    	  -no-emul-boot \
	    	  -isohybrid-gpt-basdat \
	    	  -volid KOOMPILIVE \
	    	-o koompilive.iso work/live
        "#
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
