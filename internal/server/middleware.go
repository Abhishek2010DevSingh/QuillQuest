package server

import (
	"net/http"
	"os"

	"github.com/gorilla/handlers"
)

type Middleware func(http.Handler) http.Handler

func CreateStack(xs ...Middleware) Middleware {
	return func(next http.Handler) http.Handler {
		for i := len(xs) - 1; i >= 0; i-- {
			x := xs[i]
			next = x(next)
		}

		return next
	}
}

func LoggingMiddleware() Middleware {
	return func(h http.Handler) http.Handler {
		return handlers.LoggingHandler(os.Stdout, h)
	}
}
