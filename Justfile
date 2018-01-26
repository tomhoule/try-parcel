# Compile proto files for the Rust server
codegen:
    protoc \
        --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
        --rust_out=src/rpc \
        --grpc_out=src/rpc \
        proto/*
    protoc \
      --plugin=protoc-gen-ts=./frontend/node_modules/.bin/protoc-gen-ts \
      --js_out=import_style=commonjs,binary:frontend/src/rpc \
      --ts_out=service=true:frontend/src/rpc \
      -I ./proto \
      proto/*.proto

start-proxy:
  grpcwebproxy \
    --backend_addr=localhost:4443 \
    --server_http_tls_port=4444 \
    --server_tls_cert_file=etc/localhost.crt \
    --server_tls_key_file=etc/localhost.key \
    --backend_tls=false
