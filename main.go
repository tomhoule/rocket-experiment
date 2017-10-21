package main

import (
  "flag"
  "net/http"

  "github.com/golang/glog"
  "golang.org/x/net/context"
  "github.com/grpc-ecosystem/grpc-gateway/runtime"
  "google.golang.org/grpc"

  gw "./proto"
)

var (
  repoEndpoint = flag.String("repo_endpoint", "localhost:9090", "endpoint of YourService")
)

func run() error {
  ctx := context.Background()
  ctx, cancel := context.WithCancel(ctx)
  defer cancel()

  mux := runtime.NewServeMux()
  opts := []grpc.DialOption{grpc.WithInsecure()}
  err := gw.RegisterEthicsRepositoryHandlerFromEndpoint(ctx, mux, *repoEndpoint, opts)
  if err != nil {
    return err
  }

  glog.Info("grpc gateway listening on 8008")
  return http.ListenAndServe(":8008", mux)
}

func main() {
  flag.Parse()
  defer glog.Flush()

  if err := run(); err != nil {
    glog.Fatal(err)
  }
}
