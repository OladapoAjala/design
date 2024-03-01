package checkoutserver

import (
	"context"
	"fmt"
	"log"
	"strconv"

	"github.com/OladapoAjala/design/rate-limiter/checkout/proto/checkout"
	"github.com/bradfitz/gomemcache/memcache"
	"google.golang.org/grpc/peer"
)

type Server struct {
	checkout.UnimplementedCheckouterServer
	Cache *memcache.Client
}

var _ checkout.CheckouterServer = new(Server)

func (s *Server) Checkout(ctx context.Context, req *checkout.CheckoutRequest) (*checkout.CheckoutResponse, error) {
	peer, ok := peer.FromContext(ctx)
	if !ok {
		return nil, fmt.Errorf("unable to extract client info")
	}
	log.Printf("peer: %v\n", peer)

	_, err := s.Cache.Get(peer.LocalAddr.String())
	if err != nil {
		err := s.Cache.Set(&memcache.Item{
			Key:        peer.LocalAddr.String(),
			Value:      []byte(strconv.Itoa(1)),
			Expiration: 10, // time in seconds
		})
		if err != nil {
			log.Println(err)
			return nil, err
		}
	}

	item, err := s.Cache.Get(peer.LocalAddr.String())
	if err != nil {
		return nil, err
	}
	log.Printf("current value: %s", item.Value)

	count, err := strconv.Atoi(string(item.Value))
	if err != nil {
		return nil, err
	}
	if count >= 10 {
		log.Println("rate limit exceeded")
		return nil, fmt.Errorf("rate limit exceeded")
	}

	err = s.Cache.Set(&memcache.Item{
		Key:        peer.LocalAddr.String(),
		Value:      []byte(strconv.Itoa(count + 1)),
		Expiration: 10, // time in seconds
	})
	if err != nil {
		return nil, err
	}

	return &checkout.CheckoutResponse{
		Cost: 10,
	}, nil
}
