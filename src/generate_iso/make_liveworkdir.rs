pub use run_script::{run_script, ScriptOptions};

pub fn liveworkdir() {

    println!("Preparing iso workdir....");

    let (code, output, error) = run_script::run_script!(
        r#"

        mkdir -p "rootfs" "work"
        isolinux_files = "chain.c32 isolinux.bin ldlinux.c32 libutil.c32 reboot.c32 menu.c32 libcom32.c32 poweroff.c32"
	    rm -fr work/live
	    mkdir -p work/live/koompi
	    mkdir -p work/live/isolinux
	    mkdir -p work/live/boot

	   	cp files/chain.c32 work/live/isolinux/
	   	cp files/isolinux.bin work/live/isolinux/
	   	cp files/ldlinux.c32 work/live/isolinux/
	   	cp files/libutil.c32 work/live/isolinux/
	   	cp files/reboot.c32 work/live/isolinux/
	   	cp files/menu.c32 work/live/isolinux/
	   	cp files/libcom32.c32 work/live/isolinux/
	   	cp files/poweroff.c32 work/live/isolinux/

	    cp files/isolinux.cfg work/live/isolinux
	    [ -d liverootfs ] && cp -Ra liverootfs "work/live"
        "#
        )
        .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
