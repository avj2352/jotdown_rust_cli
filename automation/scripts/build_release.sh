# Script to build CLI release

# ..Initialize
echo "Initializing clean build.."
cd ../../
rm -rf release

# ..Create build
cargo build --release
echo "Created Jotdown release build!"
echo "Creating release folder.."
mkdir release

# ..Package
echo "Packaging executable and release information.."
cp ./target/release/jotdown release/jd
cp ./*.md release/

# ..done!
echo "Packaging completed !!"