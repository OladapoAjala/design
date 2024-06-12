package main

import (
	"log"
	"net"

	"github.com/OladapoAjala/design/rate-limiter/checkout/pkgs/checkoutserver"
	"github.com/OladapoAjala/design/rate-limiter/checkout/proto/checkout"
	"google.golang.org/grpc"
)

func main() {
	server := new(checkoutserver.Server)

	lis, err := net.Listen("tcp", "0.0.0.0:3000")
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
