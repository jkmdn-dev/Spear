# Overview
 Spear is a high-performance legal chess move generation library written in Rust, designed to enhance the capabilities of my chess engine, Javelin. This library is optimized for speed and efficiency, making it an ideal choice for developers looking to integrate powerful move generation into their chess applications. 
# Perft Results
 Following tests are conducted on Ryzen 9 7950x.

| Position | Depth | Nodes | Time | Nps |
| :-: | :-: | :-: | :-: | :-: |
| Start Position | 7 | 3.195b | 7.14s | 447.54m |
| Kiwipete | 6 | 8.031b | 13.29s | 603.97m |
| Position 3 | 8 | 3.009b | 7.06s | 426.01m |
| Position 4 | 6 | 706.04m | 1.12s | 628.71m |
| Position 5 | 6 | 3.048b | 5.55s | 548.33m |
| Position 6 | 6 | 6.923b | 10.77s | 642.27m |

| Position | Depth | Nodes | Time | Nps |
| :-: | :-: | :-: | :-: | :-: |
| Start Position | 6 | 119.06m | 0.78s | 152.25m |
| Kiwipete | 5 | 193.69m | 1.14s | 169.60m |
| Position 3 | 7 | 178.63m | 1.22s | 146.42m |
| Position 4 | 5 | 15.83m | 0.10s | 155.22m |
| Position 5 | 5 | 89.94m | 0.54s | 165.94m |
| Position 6 | 5 | 164.07m | 0.95s | 171.26m |
