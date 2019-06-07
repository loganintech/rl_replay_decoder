# total=0
# won=0
# lost=0

# for filename in sample_replays/*;
# do
#     ./target/release/rl_replay $filename;
#     total=$((total+1))
#     if [ $? -eq 1 ]; then
#         echo "Won!"
#         won=$((won+1))
#     fi
#     if [ $? -ne 1 ]; then
#         echo "Lost"
#         lost=$((lost+1))
#     fi
# done

# echo "Won $won"

import os
import subprocess

won = 0
lost = 0
total = 0
for subdir in os.walk("sample_replays/"):
    for file in subdir[2]:
        # print(file)
        proc = subprocess.run(["./target/release/rl_replay", "sample_replays/" + file])
        total += 1
        if proc.returncode == 1:
            won += 1
        elif proc.returncode == 2:
            lost += 1

print("Won {}%".format((won / total) * 100))
