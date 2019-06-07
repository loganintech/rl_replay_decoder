use std::env::args;
use std::fs::File;
use std::io::Read;

use rl_replay_lib::{header::Property, parse, Replay};

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

    // let replay: Replay = dbg!(replay);

    let mut team_zero = 0;
    let mut team_one = 0;
    let mut im_on_team_zero = false;

    for prop in replay.header.game_properties {
        if prop.name == "Goals" {
            match prop.prop {
                Property::Array(goals) => {
                    for goal in goals.windows(4).step_by(4) {
                        let frame = if let Property::Int(val) = goal[0].prop {
                            val
                        } else {
                            eprintln!("{:?}", goal[0].prop);
                            unreachable!()
                        };
                        let player = if let Property::Str(val) = goal[1].prop {
                            val.to_string()
                        } else {
                            eprintln!("{:?}", goal[1].prop);
                            unreachable!()
                        };
                        let team = if let Property::Int(val) = goal[2].prop {
                            val
                        } else {
                            eprintln!("{:?}", goal[2].prop);
                            unreachable!()
                        };

                        im_on_team_zero =
                            im_on_team_zero || (player.contains("JewsOfHazard") && team != 1);

                        if team == 1 {
                            team_one += 1;
                        } else {
                            team_zero += 1;
                        }

                        // println!("[{}] Goal! {} - {}", frame, player, team);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    if (im_on_team_zero && team_zero > team_one) || (!im_on_team_zero && team_one > team_zero) {
        // println!("I won!");
        std::process::exit(1);
    } else {
        // println!("I lost!");
        std::process::exit(2);
    }
}
