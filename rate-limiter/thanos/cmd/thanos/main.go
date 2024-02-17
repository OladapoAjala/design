package main

import (
	"log"
	"net"

	"github.com/OladapoAjala/design/rate-limiter/thanos/pkgs/thanosserver"
	"github.com/OladapoAjala/design/rate-limiter/thanos/proto/thanos"
	"google.golang.org/grpc"
)

func main() {
	lis, err := net.Listen("tcp", ":8088")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := grpc.NewServer()
	thanos.RegisterCheckouterServer(s, new(thanosserver.Server))

	log.Printf("server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
