#!/bin/bash

SCRIPT_PATH=$(realpath `dirname "$0"`)
SCRIPT="$SCRIPT_PATH"/$(basename "$0")

if ! [ -d "$SCRIPT_PATH"/built ]; then mkdir "$SCRIPT_PATH"/built; fi
set -beuo pipefail

RED="\033[0;31m"
CLS="\033[0m"

__croi_exit() {
    echo -e "\nI sense interrupt\nI exit now\n"
    exit 1
}

if ! command -v flutter_rust_bridge_codegen &>/dev/null; then
    echo -e "${RED}You need to run${CLS} \`cargo install flutter_rust_bridge_codegen\`"
    exit 1
fi

trap __croi_exit SIGINT

case "${1:-}" in
    "linux")
        "${SCRIPT}" icon
        "${SCRIPT}" licenses
        "${SCRIPT}" generate
        flutter build linux --release --verbose
        if ! [ -f "${SCRIPT_PATH}/built/appimagetool-x86_64.AppImage" ]; then
            wget -O "${SCRIPT_PATH}/built/appimagetool-x86_64.AppImage" \
                https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage
            chmod +x "${SCRIPT_PATH}/built/appimagetool-x86_64.AppImage"
        fi

        if [ -d "${SCRIPT_PATH}/built/grace.AppDir" ]; then
            rm -rf "${SCRIPT_PATH}/built/grace.AppDir"
        fi

        cp -r "${SCRIPT_PATH}/build/linux/x64/release/bundle" "${SCRIPT_PATH}/built/grace.AppDir"
        cp "${SCRIPT_PATH}/assets/AppRun" "${SCRIPT_PATH}/built/grace.AppDir"
        cp "${SCRIPT_PATH}/assets/Grace.desktop" "${SCRIPT_PATH}/built/grace.AppDir"
        cp "${SCRIPT_PATH}/assets/icons/icon.png" "${SCRIPT_PATH}/built/grace.AppDir"
        "${SCRIPT_PATH}/built/appimagetool-x86_64.AppImage" \
            "${SCRIPT_PATH}/built/grace.AppDir" \
            "${SCRIPT_PATH}/built/Grace.AppImage"
        ;;
    "apk")
        "${SCRIPT}" icon
        "${SCRIPT}" licenses
        "${SCRIPT}" generate
        flutter build apk \
            --release \
            --verbose \
            --split-per-abi \
            --obfuscate \
            --split-debug-info="${SCRIPT_PATH}"/debug_info

        cp -fv "${SCRIPT_PATH}"/build/app/outputs/flutter-apk/*.apk "${SCRIPT_PATH}"/built
        "${SCRIPT}" sign
        ;;
    "run")
        rm -r "${SCRIPT_PATH}"/lib/rust/
        "${SCRIPT}" licenses
        "${SCRIPT}" generate
        flutter run --verbose
        ;;
    "clean")
        pushd "${SCRIPT_PATH}"/rust
        cargo clean
        popd
        flutter clean
        rm "${SCRIPT_PATH}"/rust/src/frb_generated.rs
        rm -r "${SCRIPT_PATH}"/lib/rust/
        ;;
    "deldb")
        rm -v "$HOME/.local/share/com.croisen.cvsu.exam_checker/exam_checker.db"
        ;;
    "icon")
        if [ "${SCRIPT_PATH}"/assets/icons/icon.png -nt "${SCRIPT_PATH}"/assets/icons/icon-old.png ]
        then
            cp -fv "${SCRIPT_PATH}"/assets/icons/icon.png "${SCRIPT_PATH}"/assets/icons/icon-old.png
            dart run flutter_launcher_icons
        fi
        ;;
    "rename")
        if [ -z "${2:-}" ]; then
            echo "Usage: $0 rename 'com.new.org.name'"
            exit 1
        fi

        echo -e "[${RED}CROI${CLS}]: This will error out as flutter made some changes to where the linux files are"
        echo -e "[${RED}CROI${CLS}]: Changes must be done manually at ${SCRIPT_PATH}/linux/CMakeLists.txt on the APPLICATION_ID variable"
        dart run rename setBundleId \
            --targets android,ios,macos,windows,web \
            --value "$2"
        ;;
    "cat")
        find "${SCRIPT_PATH}"/lib "${SCRIPT_PATH}"/rust/src \
            \( -not -path "${SCRIPT_PATH}"/lib/rust/\* \) -and \
            \( -not -iname "licenses.g.dart" \) -and \
            \( -not -iname "frb_generated.*" \) -and \
            \( -iname "*.dart" -or -name "*.rs" \) \
            | sort \
            | xargs -I{} sh -c "echo {}:; cat {}; echo -ne '\x0c'"
        ;;
    "wc")
        find "${SCRIPT_PATH}"/lib "${SCRIPT_PATH}"/rust/src \
            \( -not -path "${SCRIPT_PATH}"/lib/rust/\* \) -and \
            \( -not -iname "licenses.g.dart" \) -and \
            \( -not -iname "frb_generated.*" \) -and \
            \( -iname "*.dart" -or -name "*.rs" \) \
            | sort \
            | xargs wc -l
        ;;
    "raw-wc")
        find "${SCRIPT_PATH}"/lib "${SCRIPT_PATH}"/rust/src \
            \( -iname "*.dart" -or -name "*.rs" \) \
            | sort \
            | xargs wc -l
        ;;
    "licenses")
        dart run dart_pubspec_licenses:generate -o "${SCRIPT_PATH}"/lib/utils/licenses.g.dart
        ;;
    "sign")
        if [ -f "${SCRIPT_PATH}"/release.jks ]; then
            read -r -s -p "Enter keystore password: " PASS
            for file in "${SCRIPT_PATH}"/built/*.apk; do
                echo "$PASS" | apksigner sign --ks "${SCRIPT_PATH}"/release.jks "$file"
            done
        fi
        ;;
    "generate")
        # FRB_DEBUG_SKIP_SANITY_CHECK_CLASS_NAME_DUPLICATES=1 \
        # RUST_LOG=debug \
            flutter_rust_bridge_codegen generate
        ;;
    "run-rust")
        pushd "${SCRIPT_PATH}"/rust
        cargo run --release --bin rust-lib-grace
        popd
        ;;
    "run-rustr")
        pushd "${SCRIPT_PATH}"/rust
        cargo run --release -Z build-std=core,alloc,std,panic_abort --bin rust-lib-grace
        popd
        ;;
    "run-rustd")
        pushd "${SCRIPT_PATH}"/rust
        RUST_BACKTRACE=1 cargo run -Z build-std=core,alloc,std,panic_abort --bin rust-lib-grace
        popd
        ;;
    *)
        echo "Usage: $0 {linux,apk,run,clean,icon,deldb,rename,cat}"
        echo "linux    - build linux executable on release mode"
        echo "apk      - build android apk on release mode (and signs it if release.jks exists)"
        echo "run      - runs the app on debug mode"
        echo "clean    - deletes the built files"
        echo "icon     - builds the icon if the file in assets/icons/icon.png is new"
        echo "deldb    - deletes the db at a known place on linux"
        echo "rename   - renames the app bundle id"
        echo "cat      - echoes out the contents of all dart files under lib to stdout"
        echo "wc       - gets the line count of all dart and rust files (except generated files)"
        echo "raw-wc   - gets the line count of all dart and rust files"
        echo "licenses - generates the licenses of the dependencies used"
        echo "sign     - signs the apk if the apks exist and release.jks exists"
        echo "generate - generates code for rust <-> flutter bridge"
        exit 1
        ;;
esac
