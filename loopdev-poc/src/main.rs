use clap::Parser;
use loopdev::LoopControl;
use nix::errno;
use serde::Deserialize;
use std::{fs::File, path::Path, process::Command};

#[derive(Parser,Default,Debug)]
struct Arguments {
    backing_file: String,
    size: i64,
}

#[derive(Deserialize, Debug)]
struct LoopedDevices {
    name: String,
    #[serde(rename = "back-file")]
    backing_file: String,
}

#[derive(Deserialize, Debug)]
struct LosetupOutput {
    loopdevices: Vec<LoopedDevices>,
}

fn list_looped_devices() -> Vec<LoopedDevices> {
    let mut cmd = Command::new("losetup");
    cmd.args(["--list", "--json", "-O", "NAME,BACK-FILE"]);
    let output = cmd.output().expect("failed to list devices");

    if output.stdout.is_empty() {
        Vec::new()
    } else {
        serde_json::from_slice::<LosetupOutput>(&output.stdout)
            .unwrap()
            .loopdevices
    }
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();
    let file_path = Path::new(&args.backing_file);

    let devices = list_looped_devices();
    for elem in devices.iter() {
        if file_path.eq(Path::new(&elem.backing_file)) {
            println!("backing file {} already mounted on {}", elem.backing_file, elem.name);
            return Ok(());
        }
        println!(
            " name:{} backing_file:{}",
            elem.name,
            elem.backing_file
        )
    }

    // Note: File::create_new is reported unstable
    if file_path.exists() {
        // abort if the backing file exists
        println!("cowardly refusing to use existing file");
        return Err(errno::Errno::from(errno::Errno::EEXIST).into());
    } else {
        // create the backing file
        let f = File::create(file_path)?;
        f.sync_all()?;
    }
    // now truncate the backing file to the required size
    let res = nix::unistd::truncate(file_path, args.size);
    match res {
        Err(err) => {
            println!("failed to create file {} {}", err, file_path.display());
            return Err(err.into());
        }
        Ok(_) => {
            let lc = LoopControl::open().unwrap();
            let ld = lc.next_free().unwrap();
            println!("{}", ld.path().unwrap().display());
            ld.attach_file(file_path).unwrap();
        }
    }
    Ok(())
}
