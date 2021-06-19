pub use run_script::{run_script, ScriptOptions};

pub fn squashfs() {
    println!("Squashing filesystem...");

    let (code, output, error) = run_script::run_script!(
        r#"
        source config
	    mksquashfs $ROOTFS "$LIVEWDIR"/koompi/rootfs.sfs \
	    	-b 1048576 -comp zstd \
	    	-e $ROOTFS/tmp/* 2>/dev/null
        "#
        )
        .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);

}
