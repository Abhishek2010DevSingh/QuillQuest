package ai

import (
	"context"
	"fmt"
	"os"
	"sync"

	"github.com/google/generative-ai-go/genai"
	"google.golang.org/api/option"
)

var (
	Client *genai.Client
	Ctx    context.Context
	Once   sync.Once
)

func Init() error {
	apiKey := os.Getenv("Gemini_Api_Key")
	if apiKey == "" {
		return fmt.Errorf("'Gemini_Api_Key' is null")
	}
	var err error
	Once.Do(func() {
		Ctx = context.Background()
		Client, err = genai.NewClient(Ctx, option.WithAPIKey(apiKey))
		if err != nil {
			err = fmt.Errorf("failed to create Gemini client: %w", err)
		}
	})
	return err
}
