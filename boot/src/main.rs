use std::{
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let mut args = std::env::args().skip(1);

    let kernel_binary_path = {
        let path = PathBuf::from(args.next().unwrap());
        path.canonicalize().unwrap()
    };

    let no_boot = if let Some(arg) = args.next() {
        match arg.as_str() {
            "--no-run" => true,
            _unknown => panic!("Unexpected argument: `{}`", _unknown)
        }
    } else {
        false
    };

    let img_path = create_boot_img(&kernel_binary_path);

    if no_boot {
        println!("Boot image created at: {}", img_path.display());
        return;
    }

    let mut run_cmd = Command::new("qemu-system-x86_64");
    run_cmd
        .arg("-drive")
        .arg(format!("format=raw,file={}", img_path.display()));

    let exit_status = run_cmd.status().unwrap();
    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(1));
    }
}

fn create_boot_img(kernel_binary_path: &Path) -> PathBuf {
    let kernel_manifest_path = PathBuf::from("./Cargo.toml").canonicalize().unwrap();

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd
        .arg("--kernel-manifest")
        .arg(&kernel_manifest_path);
    build_cmd
        .arg("--kernel-binary")
        .arg(&kernel_binary_path);
    build_cmd
        .arg("--target-dir")
        .arg(kernel_manifest_path.parent().unwrap().join("target"));
    build_cmd
        .arg("--out-dir")
        .arg(kernel_binary_path.parent().unwrap());
    build_cmd.arg("--quiet");

    if !build_cmd.status().unwrap().success() {
        panic!("Build Failed!!");
    }

    let kernel_binary_name = kernel_binary_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let disk_img = kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-bios-{}.img", kernel_binary_name));

    if !disk_img.exists() {
        panic!("Disk image does not exist at: {}", disk_img.display());
    }

    disk_img
}
