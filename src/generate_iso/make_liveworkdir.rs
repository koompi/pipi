pub use run_script::{run_script, ScriptOptions};

pub fn liveworkdir() {

    println!("Preparing iso workdir....");

    let (code, output, error) = run_script::run_script!(
        r#"
        source config
        isolinux_files = "chain.c32 isolinux.bin ldlinux.c32 libutil.c32 reboot.c32 menu.c32 libcom32.c32 poweroff.c32"
	    rm -fr "$LIVEWDIR"
	    mkdir -p "$LIVEWDIR"/{koompi,isolinux,boot}
	    for file in $isolinux_files; do
	    	cp "$FILESDIR"/$file "$LIVEWDIR"/isolinux
	    done

	    cp "$FILESDIR"/isolinux.cfg "$LIVEWDIR"/isolinux
	    [ -d liverootfs ] && cp -Ra liverootfs "$LIVEWDIR"
        "#
        )
        .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
