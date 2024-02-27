package thanosserver

import (
	"context"
	"log"

	"github.com/OladapoAjala/design/rate-limiter/thanos/proto/thanos"
	"github.com/redis/go-redis/v9"
)

type Server struct {
	thanos.UnimplementedCheckouterServer
	Cache *redis.Client
}

var _ thanos.CheckouterServer = new(Server)

func (s *Server) Checkout(ctx context.Context, req *thanos.CheckoutRequest) (*thanos.CheckoutResponse, error) {
	newCtx := context.Background()
	result, err := s.Cache.Set(ctx, "key", "foo key", 0).Result()
	// err := s.Cache.Set(newCtx, "foo", "bar", 0).Err()
	if err != nil {
		log.Println(err)
		return nil, err
	}
	log.Printf("result: %v", result)

	val, err := s.Cache.Get(newCtx, "key").Result()
	if err != nil {
		return nil, err
	}
	log.Println("foo", val)

	return &thanos.CheckoutResponse{
		Cost: 10,
	}, nil
}
