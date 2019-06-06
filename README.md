A basic parser for rocket league replay files I'm developing for fun. This is not considered ready.

Example output at the time of writing this README

```
[src/main.rs:30] replay = Replay {
    header: Header {
        major: 868,
        minor: 23,
        net: Some(
            8,
        ),
        game_type: "TAGame.Replay_Soccar_TA",
        properties: [
            NamedProperty {
                name: "TeamSize",
                prop: Int(
                    2,
                ),
            },
            NamedProperty {
                name: "Team0Score",
                prop: Int(
                    2,
                ),
            },
            NamedProperty {
                name: "Team1Score",
                prop: Int(
                    6,
                ),
            },
            NamedProperty {
                name: "Goals",
                prop: Array(
                    [
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                698,
                            ),
                        },
                        NamedProperty {
                            name: "PlayerName",
                            prop: Str(
                                "JewsOfHazard[PCMR]",
                            ),
                        },
                        NamedProperty {
                            name: "PlayerTeam",
                            prop: Int(
                                0,
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                2238,
                            ),
                        },
                        NamedProperty {
                            name: "PlayerName",
                            prop: Str(
                                "Black Sloth",
                            ),
                        },
                        NamedProperty {
                            name: "PlayerTeam",
                            prop: Int(
                                1,
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                3064,
                            ),
                        },
                        NamedProperty {
                            name: "PlayerName",
                            prop: Str(
                                "jack.",
                            ),
                        },
                        NamedProperty {
                            name: "PlayerTeam",
                            prop: Int(
                                1,
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                4620,
                            ),
                        },
                        NamedProperty {
                            name: "PlayerName",
                            prop: Str(
                                "JewsOfHazard[PCMR]",
                            ),
                        },
                        NamedProperty {
                            name: "PlayerTeam",
                            prop: Int(
                                0,
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                5059,
                            ),
                        },
                        NamedProperty {
                            name: "PlayerName",
                            prop: Str(
                                "jack.",
                            ),
                        },
                        NamedProperty {
                            name: "PlayerTeam",
                            prop: Int(
                                1,
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                6696,
                            ),
                        },
                        NamedProperty {
                            name: "PlayerName",
                            prop: Str(
                                "Black Sloth",
                            ),
                        },
                        NamedProperty {
                            name: "PlayerTeam",
                            prop: Int(
                                1,
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                9180,
                            ),
                        },
                        NamedProperty {
                            name: "PlayerName",
                            prop: Str(
                                "jack.",
                            ),
                        },
                        NamedProperty {
                            name: "PlayerTeam",
                            prop: Int(
                                1,
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                9967,
                            ),
                        },
                        NamedProperty {
                            name: "PlayerName",
                            prop: Str(
                                "Black Sloth",
                            ),
                        },
                        NamedProperty {
                            name: "PlayerTeam",
                            prop: Int(
                                1,
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                    ],
                ),
            },
            NamedProperty {
                name: "HighLights",
                prop: Array(
                    [
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                698,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_50",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "None",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                1969,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_56",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "Ball_TA_16",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                2238,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_58",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "Ball_TA_16",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                3064,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_67",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "Ball_TA_16",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                4620,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_72",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "Ball_TA_16",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                5059,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_81",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "Ball_TA_16",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                6696,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_91",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "Ball_TA_16",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                7175,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_96",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "Ball_TA_26",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                8605,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_96",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "Ball_TA_26",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                9180,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_98",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "Ball_TA_26",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                        NamedProperty {
                            name: "frame",
                            prop: Int(
                                9967,
                            ),
                        },
                        NamedProperty {
                            name: "CarName",
                            prop: Name(
                                "Car_TA_105",
                            ),
                        },
                        NamedProperty {
                            name: "BallName",
                            prop: Name(
                                "Ball_TA_26",
                            ),
                        },
                        NamedProperty {
                            name: "None",
                            prop: None,
                        },
                    ],
                ),
            },
        ],
    },
    body: Body,
}
[698] Goal! JewsOfHazard[PCMR] (this one is me also this wouldn't show up in the output normally) - 0
[2238] Goal! Black Sloth - 1
[3064] Goal! jack. - 1
[4620] Goal! JewsOfHazard[PCMR] (this one is me also this wouldn't show up in the output normally) - 0
[5059] Goal! jack. - 1
[6696] Goal! Black Sloth - 1
[9180] Goal! jack. - 1
[9967] Goal! Black Sloth - 1
```
