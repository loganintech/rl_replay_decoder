use std::env::args;
use std::fs::File;
use std::io::Read;

use rl_replay_lib::{parse, Replay, header::Property};

fn main() {
    let mut file = File::open(args().nth(1).expect("You must include a replay to parse."))
        .expect("Couldn't open file for reading.");
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)
        .expect("Couldn't read bytes of file.");

    let (rest, replay) = parse(&bytes).unwrap_or_else(|e| {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    });

    let replay: Replay = dbg!(replay);
    for prop in replay.header.properties {
        if prop.name == "Goals" {
            match prop.prop {
                Property::Array(goals) => {
                    for goal in goals.windows(4).step_by(4) {
                        println!(
                            "[{}] Goal! {} - {}",
                            if let Property::Int(val) = goal[0].prop {
                                val
                            } else {
                                eprintln!("{:?}", goal[0].prop);
                                unreachable!()
                            },
                            if let Property::Str(val) = goal[1].prop {
                                val
                            } else {
                                eprintln!("{:?}", goal[1].prop);
                                unreachable!()
                            },
                            if let Property::Int(val) = goal[2].prop {
                                val
                            } else {
                                eprintln!("{:?}", goal[2].prop);
                                unreachable!()
                            },
                        );
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
