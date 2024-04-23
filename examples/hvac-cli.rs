use clap::{Parser, Subcommand};
use rbroadlink::{traits::DeviceTrait, Device};

#[derive(Parser)]
#[clap(about, version)]
struct Args {
    #[clap(subcommand)]
    command: RunMode,
}

#[derive(Subcommand, Clone, PartialEq)]
enum RunMode {
    /// show air conditioner state
    Status,
    /// toggle power state
    Toggle,
    /// power ON air conditioner
    TurnOn,
    /// power OFF air conditioner
    TurnOff,
}

fn main() {
    let args = Args::parse();

    println!(">>> autodiscovering broadlink devices...");
    let discovered = Device::list(None).expect("Could not enumerate devices!");
    for device in discovered {
        println!(">>> device authentication ...");
        let addr = device.get_info().address;
        println!(">>> device at {} => {}", addr, device);

        let hvac = match device {
            Device::Hvac { hvac } => hvac,
            _ => {
                return;
            }
        };

        if args.command == RunMode::Status {
            println!(">>> get_info");
            let ac_info = hvac.get_info().unwrap();
            println!("Current power state: {}", ac_info.power);
            println!("Ambient temperature: {:.1}", ac_info.get_ambient_temp());
        } else {
            println!(">>> get_state");
            let mut state = hvac.get_state().unwrap();
            println!("Current state: {:?}", state);

            // Setting desired mode according to command line argument
            if args.command == RunMode::Toggle {
                state.power = !state.power;
            } else if args.command == RunMode::TurnOn {
                state.power = true;
            } else if args.command == RunMode::TurnOff {
                state.power = false;
            }

            println!(">>> set_state");
            let response = hvac.set_state(&mut state).unwrap();
            println!(">>> device response {:02x?}", response);
        }
    }
}
