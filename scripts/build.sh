BLUE='\033[0;34m'
NC='\033[0m' # No Color

build_bundler () {
    printf " ${BLUE}Building Platform${NC}: Linux\n"
    cargo build -p onepack_bundler --release --target x86_64-unknown-linux-gnu
    printf " ${BLUE}Building Platform${NC}: Windows\n"
    cargo build -p onepack_bundler --release --target x86_64-pc-windows-gnu
}

build_runtime () {
    printf " ${BLUE}Building Platform${NC}: Linux\n"
    cargo build -p onepack_runtime --release --target x86_64-unknown-linux-gnu
    printf " ${BLUE}Building Platform${NC}: Windows\n"
    cargo build -p onepack_runtime --release --target x86_64-pc-windows-gnu
    cp ./target/x86_64-unknown-linux-gnu/release/onepack_runtime crates/onepack_bundler/src/runtimes/onepack_runtime_linux
    cp ./target/x86_64-pc-windows-gnu/release/onepack_runtime.exe  crates/onepack_bundler/src/runtimes/onepack_runtime_windows.exe
}

case $1 in

  "bundler")
    build_bundler
    ;;

  "runtime")
    build_runtime
    ;;

  *)
    build_runtime
    build_bundler
    ;;
esac
