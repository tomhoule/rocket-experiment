clean-proto:
    rm -rf js/src/rpc
    mkdir -p js/src/rpc

compile-proto: clean-proto
    protoc \
        --rust_out=src/rpc \
        --grpc_out=src/rpc \
        --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
        *.proto
    protoc \
        --plugin=protoc-gen-ts=`which protoc-gen-ts` \
        --js_out=import_style=commonjs,binary:js/src/rpc \
        --ts_out=service=true:js/src/rpc \
        -I . \
        *.proto

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
