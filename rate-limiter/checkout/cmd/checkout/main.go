package main

import (
	"log"
	"net"

	"github.com/OladapoAjala/design/rate-limiter/checkout/pkgs/checkoutserver"
	"github.com/OladapoAjala/design/rate-limiter/checkout/proto/checkout"
	"github.com/bradfitz/gomemcache/memcache"
	"google.golang.org/grpc"
)

func main() {
	server := &checkoutserver.Server{
		Cache: memcache.New("cache-mcrouter:5000"),
	}

	lis, err := net.Listen("tcp", "127.0.0.1:3000")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()
	checkout.RegisterCheckouterServer(grpcServer, server)

	log.Printf("server listening at %v", lis.Addr())
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
