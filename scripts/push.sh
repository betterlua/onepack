# bash scripts/build.sh
bash scripts/build.sh runtime

cp ./target/x86_64-unknown-linux-gnu/release/onepack_runtime crates/onepack_bundler/src/runtimes/onepack_runtime_linux
cp ./target/x86_64-pc-windows-gnu/release/onepack_runtime.exe  crates/onepack_bundler/src/runtimes/onepack_runtime_windows.exe

git add .
git commit -m "$1"
git push