package main

import (
	"log"
	"net"

	"github.com/OladapoAjala/design/rate-limiter/thanos/pkgs/thanosserver"
	"github.com/OladapoAjala/design/rate-limiter/thanos/proto/thanos"
	"github.com/bradfitz/gomemcache/memcache"
	"google.golang.org/grpc"
)

func main() {
	server := &thanosserver.Server{
		Cache: memcache.New("cache-mcrouter:5000"),
	}

	lis, err := net.Listen("tcp", ":8080")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()
	thanos.RegisterCheckouterServer(grpcServer, server)

	log.Printf("server listening at %v", lis.Addr())
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
