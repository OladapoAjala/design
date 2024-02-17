package thanosserver

import (
	"context"
	"fmt"

	"github.com/OladapoAjala/design/rate-limiter/thanos/proto/thanos"
)

type Server struct {
	thanos.UnimplementedCheckouterServer
}

var _ thanos.CheckouterServer = new(Server)

func (s *Server) Checkout(ctx context.Context, req *thanos.CheckoutRequest) (*thanos.CheckoutResponse, error) {
	fmt.Println("Hello dapo")
	return &thanos.CheckoutResponse{
		Cost: 10,
	}, nil
}
