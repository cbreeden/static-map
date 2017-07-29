# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cross build --target $TARGET -p $CRATE_NAME
    cross build --target $TARGET --release -p $CRATE_NAME

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --test tests --target $TARGET -p $CRATE_NAME
    cross test --test tests --target $TARGET --release -p $CRATE_NAME
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
