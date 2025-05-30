cargo build --release;
if [ $? -ne 0 ]; then
    echo "Build failed. Please check the errors above."
    exit 1
fi

./target/release/depp add js vite hono ukv;
./target/release/depp add rs itertools;
./target/release/depp add py more-itertools;
./target/release/depp remove js ukv;
