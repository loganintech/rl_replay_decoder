import os
import subprocess

won = 0
lost = 0
total = 0
for subdir in os.walk("sample_replays/"):
    for file in subdir[2]:
        print(file)
        proc = subprocess.run(["./target/release/rl_replay", "sample_replays/" + file])
        total += 1
        if proc.returncode == 1:
            won += 1
        elif proc.returncode == 2:
            lost += 1

print("Won {}%".format((won / total) * 100))
