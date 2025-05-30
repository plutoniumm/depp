cargo build --release;
if [ $? -ne 0 ]; then
    echo "Build failed. Please check the errors above."
    exit 1
fi

./target/release/depp add js vite;
./target/release/depp add rs itertools;
