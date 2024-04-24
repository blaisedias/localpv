use clap::Parser;
use serde::Deserialize;
use std::process::Command;


#[derive(Parser, Default, Debug)]
struct Arguments {
    backing_file: String,
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
    let devices = list_looped_devices();
    for elem in devices.iter() {
        if args.backing_file.eq(&elem.backing_file) {
            let res = loopdev::LoopDevice::open(elem.name.clone());
            match res {
                Err(err) => {
                    return Err(err.into());
                }
                Ok(_) => {
                    let ld = res.unwrap();
                    ld.detach()?;
                }
            }
        }
    }

    Ok(())
}
