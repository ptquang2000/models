ROOT=$(pwd)
COMPILER_FLAGS='--cfg MODELS_INTERNAL -g -C opt-level=0 -A unused'
LINK_FLAGS=''

mkdir -p build
pushd build >/dev/null

rustc --error-format=human $COMPILER_FLAGS $LINK_FLAGS $ROOT/linux_models.rs --crate-type bin
rustc --error-format=human $COMPILER_FLAGS $LINK_FLAGS $ROOT/linux_models.rs --test -o linux_models_test

popd >/dev/null
