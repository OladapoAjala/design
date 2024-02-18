package thanosserver

import (
	"context"
	"fmt"

	"github.com/OladapoAjala/design/rate-limiter/thanos/proto/thanos"
	"github.com/redis/go-redis/v9"
)

type Server struct {
	thanos.UnimplementedCheckouterServer
	Cache *redis.Client
}

var _ thanos.CheckouterServer = new(Server)

func (s *Server) Checkout(ctx context.Context, req *thanos.CheckoutRequest) (*thanos.CheckoutResponse, error) {
	fmt.Println("Hello dapo")
	return &thanos.CheckoutResponse{
		Cost: 10,
	}, nil
}
