package thanosserver

import (
	"context"
	"fmt"
	"log"
	"strconv"
	"time"

	"github.com/OladapoAjala/design/rate-limiter/thanos/proto/thanos"
	"github.com/redis/go-redis/v9"
	"google.golang.org/grpc/peer"
)

type Server struct {
	thanos.UnimplementedCheckouterServer
	Cache *redis.Client
}

var _ thanos.CheckouterServer = new(Server)

func (s *Server) Checkout(ctx context.Context, req *thanos.CheckoutRequest) (*thanos.CheckoutResponse, error) {
	peer, ok := peer.FromContext(ctx)
	if !ok {
		return nil, fmt.Errorf("unable to extract client info")
	}
	log.Printf("peer: %v\n", peer)

	oldVal, err := s.Cache.Get(ctx, peer.LocalAddr.String()).Result()
	if err == redis.Nil || oldVal == "" {
		err := s.Cache.Set(ctx, peer.LocalAddr.String(), 1, 10*time.Second).Err()
		if err != nil {
			log.Println(err)
			return nil, err
		}
	} else if err != nil {
		return nil, fmt.Errorf("unknown error %w", err)
	}

	val, err := s.Cache.Get(ctx, peer.LocalAddr.String()).Result()
	if err == redis.Nil || oldVal == "" {
		return nil, err
	}
	log.Printf("current value: %v", val)

	count, err := strconv.Atoi(val)
	if err != nil {
		return nil, err
	}
	if count >= 10 {
		log.Println("rate limit exceeded")
		return nil, fmt.Errorf("rate limit exceeded")
	}

	err = s.Cache.Set(ctx, peer.LocalAddr.String(), count+1, 10*time.Second).Err()
	if err != nil {
		return nil, err
	}

	return &thanos.CheckoutResponse{
		Cost: 10,
	}, nil
}
