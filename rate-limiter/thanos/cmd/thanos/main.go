package main

import (
	"log"
	"net"

	"github.com/OladapoAjala/design/rate-limiter/thanos/pkgs/thanosserver"
	"github.com/OladapoAjala/design/rate-limiter/thanos/proto/thanos"
	"github.com/redis/go-redis/v9"
	"google.golang.org/grpc"
)

func main() {
	rdb := redis.NewClient(&redis.Options{
		Addr:     "redis-master:6379",
		Password: "7dluNxgMY1",
		DB:       0,
	})
	server := &thanosserver.Server{
		Cache: rdb,
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
