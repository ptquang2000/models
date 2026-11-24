$COMPILER_FLAGS = @('--cfg', 'MODELS_INTERNAL', '-g', '-C', 'opt-level=0', '-A', 'unused')
$LINK_FLAGS = @('-C', 'link-args=/SUBSYSTEM:CONSOLE')

New-Item -ErrorAction Ignore -ItemType Directory -Path $PSScriptRoot/build
Push-Location $PSScriptRoot/build

rustc --error-format=short $COMPILER_FLAGS $LINK_FLAGS $PSScriptRoot/win32_models.rs --crate-type bin
rustc --error-format=short $COMPILER_FLAGS $LINK_FLAGS $PSScriptRoot/models.rs --test

Pop-Location
