package services

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

type AuthClient struct {
	baseURL string
	client  *http.Client
}

func NewAuthClient(baseURL string) *AuthClient {
	return &AuthClient{
		baseURL: baseURL,
		client:  &http.Client{Timeout: 5 * time.Second},
	}
}

func (c *AuthClient) Do(endpoint string, params map[string]string) ([]byte, error) {
	jsonData, err := json.Marshal(params)
	if err != nil {
		return nil, fmt.Errorf("marshal params: %w", err)
	}

	url := c.baseURL + endpoint
	fmt.Printf("🔍 AUTH-CLIENT: POST %s\n", url)
	fmt.Printf("🔍 AUTH-CLIENT: Payload: %s\n", string(jsonData))

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		return nil, fmt.Errorf("new request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := c.client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("http do: %w", err)
	}
	defer resp.Body.Close()

	bodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("read body: %w", err)
	}

	fmt.Printf("🔍 AUTH-CLIENT: Status %s, Body (%d bytes): %s\n", resp.Status, len(bodyBytes), string(bodyBytes))

	if resp.StatusCode >= 400 {
		return nil, fmt.Errorf("auth service error %d: %s", resp.StatusCode, string(bodyBytes))
	}

	return bodyBytes, nil
}
