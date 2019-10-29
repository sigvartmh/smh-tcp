#!/bin/sh
cargo b --release
target=$CARGO_TARGET_DIR/release/smh-tcp
sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR
sudo setcap cap_net_admin=eip $target
$target & pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
wait $pid
