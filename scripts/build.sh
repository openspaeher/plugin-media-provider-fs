rm -rf out
rm -rf plugin.tar.gz
mkdir out
cargo build --release
cp ./target/wasm32-wasip2/release/plugin_media_provider_fs.wasm ./out/plugin.wasm
cp ./plugin.toml ./out/plugin.toml
version=$(cat ./wit/version.txt)
sed -i -e "s/%VERSION%/$version/g" ./out/plugin.toml
cp -a ./resources/. ./out/resources
tar czf plugin.tar.gz -C ./out .
mv plugin.tar.gz ./out/plugin.tar.gz