# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin $CRATE_NAME --target $TARGET --release -- -C panic='abort' -C overflow-checks='no' -C debuginfo=0 -C codegen-units=1 -C lto -C opt-level='z' -C link-arg=-s

    cp target/$TARGET/release/$CRATE_NAME $stage/ || cp target/$TARGET/release/$CRATE_NAME.exe $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
