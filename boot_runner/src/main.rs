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

    let did_create_img = create_boot_img(&kernel_binary_path);
    if !did_create_img {
        eprintln!("Could not create the boot ISO");
        std::process::exit(1);
    }

    if no_boot {
        println!("Boot image created!");
        return;
    }

    let mut run_cmd = Command::new("qemu-system-x86_64");
    run_cmd
        .arg("-cdrom").arg("./target/_qemu_img")
        .arg("-debugcon").arg("stdio")
        .arg("-m").arg("1G");

    let exit_status = run_cmd.status().unwrap();
    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(1));
    }
}

fn create_iso_root() {
    let mut mkdir_cmd = Command::new("mkdir");
    mkdir_cmd
        .arg("-p")
        .arg("./target/iso_root");

    mkdir_cmd.status().expect("Could not create the root directory for the ISO root");
}

fn copy_kernel_obj(kernel_binary_path: &Path) -> bool {
    let mut cp_cmd = Command::new("cp");
    cp_cmd
        .arg(format!("{}", kernel_binary_path.display()))
        .arg("./target/iso_root/");

    cp_cmd.status().expect("Could not copy the kernel image into the ISO root").success()
}

fn build_limine_bin() -> bool {
    let mut build_cmd = Command::new("make");
    build_cmd
        .arg("-C")
        .arg("./vendor/limine");

    build_cmd.status().expect("Could not execute limine build command").success()
}

fn copy_limine_data() -> bool {
    let mut cp_cmd = Command::new("cp");
    cp_cmd
        .arg("./vendor/limine/limine-cd.bin")
        .arg("./vendor/limine/limine-eltorito-efi.bin")
        .arg("./vendor/limine/limine.sys")
        .arg("./target_configs/limine.cfg")
        .arg("./target/iso_root");

    cp_cmd.status().expect("Could not execute limine-cfg copy").success()
}

fn burn_iso() -> bool {
    let mut burn_cmd = Command::new("xorriso");
    burn_cmd
        .arg("-as").arg("mkisofs")
        .arg("-b").arg("limine-cd.bin")
        .arg("-no-emul-boot")
        .arg("-boot-load-size").arg("4")
        .arg("-boot-info-table")
        .arg("--efi-boot").arg("limine-eltorito-efi.bin")
        .arg("-efi-boot-part")
        .arg("--efi-boot-image")
        .arg("--protective-msdos-label")
        .arg("./target/iso_root")
        .arg("-o").arg("./target/_qemu_img");

    if !burn_cmd.status().expect("Could not run ISO burn command").success() {
        return false;
    }

    let mut install_cmd = Command::new("./vendor/limine/limine-install");
    install_cmd.arg("./target/_qemu_img");

    if !install_cmd.status().expect("Could not run limine install command").success() {
        return false;
    }

    true
}

fn create_boot_img(kernel_binary_path: &Path) -> bool {
    create_iso_root();

    let did_copy_kernel_obj = copy_kernel_obj(kernel_binary_path);
    if !did_copy_kernel_obj {
        eprintln!("Kernel image copy failed!");
        return false;
    }

    let did_build_limine = build_limine_bin();
    if !did_build_limine {
        eprintln!("Could not build Limine");
        return false;
    }

    let did_copy_limine_bins = copy_limine_data();
    if !did_copy_limine_bins {
        eprintln!("Could not copy Limine data");
        return false;
    }

    let did_burn_iso = burn_iso();
    if !did_burn_iso {
        eprintln!("Could not burn the boot ISO");
        return false;
    }

    true
}

