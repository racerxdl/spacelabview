package api

import (
	"bytes"
	"crypto/hmac"
	"crypto/rand"
	"crypto/sha1"
	"encoding/base64"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"net/url"
	"strings"
	"time"
)

type SpaceAPI struct {
	url    string
	secret []byte
}

func MakeAPI(baseUrl, secret string) (*SpaceAPI, error) {
	sec, err := base64.StdEncoding.DecodeString(secret)
	if err != nil {
		return nil, InvalidSecretError
	}
	s := &SpaceAPI{
		secret: sec,
		url:    baseUrl,
	}
	err = s.Ping()
	if err != nil {
		return nil, err
	}

	return s, nil
}

func (s *SpaceAPI) Delete(resource string, params map[string]string) (string, error) {
	return s.Req(resource, "DELETE", params, nil)
}

func (s *SpaceAPI) Get(resource string, params map[string]string) (string, error) {
	return s.Req(resource, "GET", params, nil)
}

func (s *SpaceAPI) Post(resource string, params map[string]string, body string) (string, error) {
	return s.Req(resource, "POST", params, []byte(body))
}

func (s *SpaceAPI) Req(resource string, method string, params map[string]string, body []byte) (string, error) {
	methodUrl := "/vrageremote" + resource
	date := strings.Replace(time.Now().UTC().Format(time.RFC1123), "UTC", "GMT", -1)
	nonce := getNonce()

	reqUrl := methodUrl + "?"
	for k, v := range params {
		escapedV := url.QueryEscape(v)
		escapedK := url.QueryEscape(k)
		reqUrl += fmt.Sprintf("%s=%s&", escapedK, escapedV)
	}
	reqUrl = reqUrl[:len(reqUrl)-1] // Remove leading & or leading ? if no params
	message := fmt.Sprintf("%s\r\n%s\r\n%s\r\n", reqUrl, nonce, date)

	hasher := hmac.New(sha1.New, s.secret)
	_, err := hasher.Write([]byte(message))
	if err != nil {
		return "", err
	}
	computedHash := base64.StdEncoding.EncodeToString(hasher.Sum(nil))

	var b io.Reader

	if len(body) > 0 {
		b = bytes.NewReader(body)
	}

	req, err := http.NewRequest(method, s.url+reqUrl, b)
	if err != nil {
		panic(err)
	}
	req.Header.Add("Date", date)
	req.Header.Add("Authorization", fmt.Sprintf("%s:%s", nonce, computedHash))

	c := http.Client{}
	resp, err := c.Do(req)
	if err != nil {
		return "", err
	}

	if resp.StatusCode == 403 {
		return "", AccessDeniedError
	}

	if resp.StatusCode != 200 {
		return "", SpaceError(fmt.Sprintf("invalid response: %s", resp.Status))
	}

	data, err := ioutil.ReadAll(resp.Body)
	return string(data), err
}

func (s *SpaceAPI) Ping() error {
	_, err := s.Get("/v1/server/ping", nil)
	return err
}

func getNonce() string {
	data := make([]byte, 20)
	_, err := rand.Read(data)
	if err != nil {
		panic(err)
	}
	return base64.StdEncoding.EncodeToString(data)
}
