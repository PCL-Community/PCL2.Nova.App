package network

import (
	"bytes"
	"io"
	"net/http"
)

type Network struct{}

func BaseRequest(url string, reqMethod string, header map[string]string, data []byte) ([]byte, error) {
	if reqMethod == "" {
		reqMethod = http.MethodGet
	}
	if header == nil {
		header = make(map[string]string)
	}
	header["Content-Type"] = "application/json;charset=utf-8"
	req, err := http.NewRequest(reqMethod, url, bytes.NewBuffer(data))
	if err != nil {
		return nil, err
	}
	for k, v := range header {
		req.Header.Set(k, v)
	}
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer func(Body io.ReadCloser) {
		err = Body.Close()
		if err != nil {
			panic(err)
		}
	}(resp.Body)
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	return body, nil
}
