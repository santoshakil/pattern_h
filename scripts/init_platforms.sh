#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
TEMPLATES_DIR="$ROOT_DIR/templates/platform"
APP_DIR=$(find "$ROOT_DIR/flutter/apps" -maxdepth 1 -mindepth 1 -type d | head -1)
if [ -z "$APP_DIR" ]; then
    echo "No app found in flutter/apps/"
    exit 1
fi

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'
ok()   { echo -e "${GREEN}[OK]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }

if ! command -v flutter &> /dev/null; then
    echo "Flutter SDK not found. Install from https://flutter.dev"
    exit 1
fi

PLATFORMS="${1:-android,ios,macos,windows,linux}"

echo "=== Initializing Platform Support ==="
echo "  Platforms: $PLATFORMS"
echo ""

cd "$APP_DIR"

APP_NAME=$(basename "$APP_DIR")
ORG=$(grep -m1 'applicationId' android/app/build.gradle.kts 2>/dev/null | sed 's/.*"\(.*\)\..*/\1/' || echo "com.example")

echo "Creating Flutter platform directories..."
flutter create --platforms "$PLATFORMS" --org "$ORG" --project-name "$APP_NAME" . > /dev/null 2>&1
ok "Flutter platform directories created"

# Android: add INTERNET permission
ANDROID_MANIFEST="$APP_DIR/android/app/src/main/AndroidManifest.xml"
if [ -f "$ANDROID_MANIFEST" ] && ! grep -q "android.permission.INTERNET" "$ANDROID_MANIFEST"; then
    sed -i.bak 's|<manifest |<manifest xmlns:tools="http://schemas.android.com/tools" |' "$ANDROID_MANIFEST" 2>/dev/null || true
    sed -i.bak '/<manifest/a\
    <uses-permission android:name="android.permission.INTERNET"/>\
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE"/>' "$ANDROID_MANIFEST"
    rm -f "$ANDROID_MANIFEST.bak"
    ok "Android: INTERNET permission added"
fi

# macOS: add network.client entitlement
for ENT_FILE in "$APP_DIR/macos/Runner/Release.entitlements" "$APP_DIR/macos/Runner/DebugProfile.entitlements"; do
    if [ -f "$ENT_FILE" ] && ! grep -q "network.client" "$ENT_FILE"; then
        sed -i.bak '/<\/dict>/i\
	<key>com.apple.security.network.client</key>\
	<true/>' "$ENT_FILE"
        rm -f "$ENT_FILE.bak"
    fi
done
if [ -f "$APP_DIR/macos/Runner/Release.entitlements" ]; then
    ok "macOS: network.client entitlement added"
fi

IFS=',' read -ra PLAT_LIST <<< "$PLATFORMS"
for plat in "${PLAT_LIST[@]}"; do
    plat=$(echo "$plat" | xargs)
    case "$plat" in
        android)
            if [ -d "$TEMPLATES_DIR/android" ]; then
                ANDROID_PKG_DIR=$(find "$APP_DIR/android/app/src/main/kotlin" -name "MainActivity.kt" -exec dirname {} \; 2>/dev/null | head -1)
                if [ -n "$ANDROID_PKG_DIR" ]; then
                    ANDROID_PKG=$(grep -m1 '^package ' "$ANDROID_PKG_DIR/MainActivity.kt" | sed 's/package //')
                    mkdir -p "$ANDROID_PKG_DIR/channels"
                    for f in "$TEMPLATES_DIR/android/"*.kt; do
                        fname=$(basename "$f")
                        if [ "$fname" = "MainActivity.kt" ]; then
                            sed "s/{{package}}/$ANDROID_PKG/g" "$f" > "$ANDROID_PKG_DIR/$fname"
                        else
                            sed "s/{{package}}/$ANDROID_PKG/g; s|{{channel_prefix}}|$ORG/$APP_NAME|g" "$f" > "$ANDROID_PKG_DIR/channels/$fname"
                        fi
                    done
                    ok "Android channels injected"
                fi
            fi
            ;;
        ios)
            if [ -d "$TEMPLATES_DIR/ios" ]; then
                for f in "$TEMPLATES_DIR/ios/"*.swift; do
                    fname=$(basename "$f")
                    sed "s|{{channel_prefix}}|$ORG/$APP_NAME|g" "$f" > "$APP_DIR/ios/Runner/$fname"
                done
                if command -v ruby &> /dev/null && ruby -e 'require "xcodeproj"' 2>/dev/null; then
                    ruby -e "
require 'xcodeproj'
proj = Xcodeproj::Project.open('$APP_DIR/ios/Runner.xcodeproj')
target = proj.targets.find { |t| t.name == 'Runner' }
group = proj.main_group.find_subpath('Runner', true)
['ChannelRegistry.swift', 'DeviceInfoChannel.swift'].each do |name|
  next if group.files.any? { |f| f.path == name }
  ref = group.new_file(name)
  target.source_build_phase.add_file_reference(ref)
end
proj.save
"
                    ok "iOS channels injected + Xcode project updated"
                else
                    warn "iOS channels injected (add ChannelRegistry.swift + DeviceInfoChannel.swift to Xcode manually)"
                fi
            fi
            ;;
        macos)
            if [ -d "$TEMPLATES_DIR/macos" ]; then
                for f in "$TEMPLATES_DIR/macos/"*.swift; do
                    fname=$(basename "$f")
                    sed "s|{{channel_prefix}}|$ORG/$APP_NAME|g" "$f" > "$APP_DIR/macos/Runner/$fname"
                done
                if command -v ruby &> /dev/null && ruby -e 'require "xcodeproj"' 2>/dev/null; then
                    ruby -e "
require 'xcodeproj'
proj = Xcodeproj::Project.open('$APP_DIR/macos/Runner.xcodeproj')
target = proj.targets.find { |t| t.name == 'Runner' }
group = proj.main_group.find_subpath('Runner', true)
['ChannelRegistry.swift', 'DeviceInfoChannel.swift'].each do |name|
  next if group.files.any? { |f| f.path == name }
  ref = group.new_file(name)
  target.source_build_phase.add_file_reference(ref)
end
proj.save
"
                    ok "macOS channels injected + Xcode project updated"
                else
                    warn "macOS channels injected (add ChannelRegistry.swift + DeviceInfoChannel.swift to Xcode manually)"
                fi
            fi
            ;;
        windows)
            if [ -d "$TEMPLATES_DIR/windows" ]; then
                RUNNER_DIR="$APP_DIR/windows/runner"
                for f in "$TEMPLATES_DIR/windows/"*.{h,cpp}; do
                    [ -f "$f" ] || continue
                    fname=$(basename "$f")
                    sed "s|{{channel_prefix}}|$ORG/$APP_NAME|g" "$f" > "$RUNNER_DIR/$fname"
                done
                FWCPP="$RUNNER_DIR/flutter_window.cpp"
                if [ -f "$FWCPP" ] && ! grep -q "channel_registry.h" "$FWCPP"; then
                    sed -i.bak '/#include "flutter\/generated_plugin_registrant.h"/a\
#include "channel_registry.h"' "$FWCPP"
                    sed -i.bak 's/RegisterPlugins(flutter_controller_->engine());/RegisterPlugins(flutter_controller_->engine());\n  RegisterChannels(flutter_controller_->engine());/' "$FWCPP"
                    rm -f "$FWCPP.bak"
                fi
                CMAKE="$RUNNER_DIR/CMakeLists.txt"
                if [ -f "$CMAKE" ] && ! grep -q "channel_registry" "$CMAKE"; then
                    sed -i.bak '/"flutter_window.cpp"/a\
  "channel_registry.cpp"\
  "device_info_channel.cpp"' "$CMAKE"
                    rm -f "$CMAKE.bak"
                fi
                ok "Windows channels injected"
            fi
            ;;
        linux)
            if [ -d "$TEMPLATES_DIR/linux" ]; then
                RUNNER_DIR="$APP_DIR/linux/runner"
                for f in "$TEMPLATES_DIR/linux/"*.{h,cc}; do
                    [ -f "$f" ] || continue
                    fname=$(basename "$f")
                    sed "s|{{channel_prefix}}|$ORG/$APP_NAME|g" "$f" > "$RUNNER_DIR/$fname"
                done
                MYAPP="$RUNNER_DIR/my_application.cc"
                if [ -f "$MYAPP" ] && ! grep -q "channel_registry.h" "$MYAPP"; then
                    sed -i.bak '/#include "flutter\/generated_plugin_registrant.h"/a\
#include "channel_registry.h"' "$MYAPP"
                    sed -i.bak 's/fl_register_plugins(FL_PLUGIN_REGISTRY(view));/fl_register_plugins(FL_PLUGIN_REGISTRY(view));\n\n  register_channels(view);/' "$MYAPP"
                    rm -f "$MYAPP.bak"
                fi
                CMAKE="$RUNNER_DIR/CMakeLists.txt"
                if [ -f "$CMAKE" ] && ! grep -q "channel_registry" "$CMAKE"; then
                    sed -i.bak '/"my_application.cc"/a\
  "channel_registry.cc"\
  "device_info_channel.cc"' "$CMAKE"
                    rm -f "$CMAKE.bak"
                fi
                ok "Linux channels injected"
            fi
            ;;
    esac
done

if [ -d "$TEMPLATES_DIR/dart" ]; then
    mkdir -p "$APP_DIR/lib/platform"
    for f in "$TEMPLATES_DIR/dart/"*.dart; do
        fname=$(basename "$f")
        sed "s|{{channel_prefix}}|$ORG/$APP_NAME|g" "$f" > "$APP_DIR/lib/platform/$fname"
    done
    ok "Dart platform wrappers created"
fi

echo ""
echo -e "${GREEN}=== Platform Setup Complete ===${NC}"
