#!/bin/sh
set -eu
cd "$(dirname "$0")"
arch=$(rustc -vV | sed -n 's/^host: //p')
cargo build --locked --target "$arch" --profile "${1:-release}" --target-dir target
app="target/L012 Servo.app"
mkdir -p "$app/Contents/MacOS"
profile=${1:-release}
if [ "$profile" = dev ]; then profile=debug; fi
cp "target/$arch/$profile/servo-panel" "$app/Contents/MacOS/servo-panel"
cp Info.plist "$app/Contents/Info.plist"
codesign --force --sign - "$app"
