# Lunar Detection

This repo is a Rust script that is designed to detect and shut down the Lunar gaming aimbot in the repo at https://github.com/zeyad-mansour/Lunar.

I wrote this script in order to learn Rust and to improve my bot detection skills. This script is not actually being used to stop anyone from cheating at fortnite or any other FSP. This is a very basic, no doubt buggy, tool that would need to be improved before it could be used in production.

I copied the [repo](https://github.com/zeyad-mansour/Lunar) into the Lunar directory in my own repo in order to modify it and to easily run my detection script against it. I modified the Lunar repo in order to be able to run it on MacOS. AFAIK the aimbots are mostly used on Windows systems, but as I do not have a Windows PC I modified the repo to be able to run and test it locally. With my modifications, the Lunar aimbot will not actually run on my MacOS system, which suits my purposes as I only need the Lunar aimbot to start without error.

The Lunar aimbot does not take any measures to avoid detection, so it is fairly easy to identify if it is running, and to shut it down. Anyone who is serious about using an aimbot and avoiding detection would need to either modify the script or else use a much more robust tool, as the gaming companies no doubt have professionals who devote themselves fulltime to stopping the aimbots.

## How to Detect the Lunar Aimbot
Follow the Lunar [instructions[](https://github.com/davidpaulmcintyre/lunar_detection/tree/main/Lunar) to install and start the aimbot, which are the same as in the [original repo](https://github.com/zeyad-mansour/Lunar).

Then install the rust dependencies for the aimbot detector
```
cargo install
cargo run
```
The rust script should detect and stop the python process running the Lunar/lunar.py script.