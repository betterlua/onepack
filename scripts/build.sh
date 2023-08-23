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