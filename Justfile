codegen: compile-proto

compile-go:
  go get
  go build
  go install

compile-proto:
  # Proxy grpc stub
  protoc -I/usr/local/include -I. \
    -I$GOPATH/src \
    -I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis \
    -Iproto \
    --go_out=plugins=grpc:. \
    ./proto/repository.proto

  # Proxy
  protoc -I/usr/local/include -I. \
    -I$GOPATH/src \
    -I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis \
    -Iproto \
    --grpc-gateway_out=logtostderr=true:. \
    ./proto/repository.proto

  # Swagger
  protoc -I/usr/local/include -I. \
    -I$GOPATH/src \
    -I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis \
    -Iproto \
    --swagger_out=logtostderr=true:. \
    ./proto/repository.proto

  # Rust
  protoc \
    -I$GOPATH/src \
    -I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis \
    -Iproto \
    --rust_out=src/rpc \
    --grpc_out=src/rpc \
    --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
    ./proto/repository.proto

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
  watchexec -c --exts rs --restart "cargo run"

watch-proto:
  watchexec -c --exts proto --restart "just compile-proto && just compile-go"

write-schema:
  diesel print-schema > src/db/schema.rs
