ROOT=$(pwd)
COMPILER_FLAGS='--cfg MODELS_INTERNAL -g -C opt-level=0 -A unused'
LINK_FLAGS=''

mkdir -p build
pushd build >/dev/null

rustc --error-format=human $COMPILER_FLAGS $LINK_FLAGS --out-dir . $ROOT/linux_models.rs --crate-type bin

popd >/dev/null
