# Script to build CLI release
echo "Initializing clean build.."
cd ../../
rm -rf release
cargo build --release
echo "Created Jotdown release build!"
echo "Creating release folder.."
mkdir release
echo "Packaging executable and release information.."
cp ./target/release/jotdown release/jd
cp ./*.md release/
echo "Packaging completed !!"