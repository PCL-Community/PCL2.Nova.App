package network

import "net/http"

func (n Network) HttpGet(url string, headers map[string]string, data []byte) ([]byte, error) {
    return BaseRequest(url, http.MethodGet, headers, data)
}
