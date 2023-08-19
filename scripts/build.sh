BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Bundler
printf "    ${BLUE}Building Platform${NC}: Linux\n"
cargo build -p onepack_runtime --release --target x86_64-unknown-linux-gnu
printf "    ${BLUE}Building Platform${NC}: Windows\n"
cargo build -p onepack_runtime --release --target x86_64-pc-windows-gnu


# Runtime
printf "    ${BLUE}Building Platform${NC}: Linux\n"
cargo build -p onepack_bundler --release --target x86_64-unknown-linux-gnu
printf "    ${BLUE}Building Platform${NC}: Windows\n"
cargo build -p onepack_bundler --release --target x86_64-pc-windows-gnu

