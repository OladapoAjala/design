package checkoutserver

import (
	"context"

	"github.com/OladapoAjala/design/rate-limiter/checkout/proto/checkout"
	"github.com/bradfitz/gomemcache/memcache"
)

type Server struct {
	checkout.UnimplementedCheckouterServer
	Cache *memcache.Client
}

var _ checkout.CheckouterServer = new(Server)

func (s *Server) Checkout(_ context.Context, _ *checkout.CheckoutRequest) (*checkout.CheckoutResponse, error) {
	return &checkout.CheckoutResponse{
		Cost: 10,
	}, nil
}
