clean-proto:
    rm -rf ts/src/rpc
    mkdir -p ts/src/rpc

compile-proto: clean-proto
    protoc \
        --rust_out=src/rpc \
        --grpc_out=src/rpc \
        --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
        -I ./proto \
        ./proto/*.proto
    protoc \
        --plugin=protoc-gen-ts=`which protoc-gen-ts` \
        --js_out=import_style=commonjs,binary:ts/src/rpc \
        --ts_out=service=true:ts/src/rpc \
        -I ./proto \
        ./proto/*.proto

start-postgres:
    sudo systemctl start postgresql

start-proxy:
    grpcwebproxy \
        --server_tls_cert_file=./misc/localhost.crt \
        --server_tls_key_file=./misc/localhost.key \
        --server_http_debug_port=8081 \
        --backend_addr=localhost:9090 \
        --backend_tls_noverify

watch:
    watchexec -c --exts rs --restart "cargo run --bin repository"

watch-web:
    watchexec -c --exts rs,hbs --restart "cargo run --bin locutions-server"

write-schema:
    diesel print-schema > src/db/schema.rs
