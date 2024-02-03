app_name="kraken"

rm $app_name
cargo build --release
mv target/release/$app_name .