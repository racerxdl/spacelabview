package spacelab

import (
	"bytes"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"net/url"
	"strings"
	"time"

	"github.com/racerxdl/spacelabview/api"
)

type API struct {
	url string
}

func MakeAPI(baseUrl string) (*API, error) {
	s := &API{
		url: baseUrl,
	}

	return s, nil
}

func (s *API) Get(resource string, params map[string]string) (string, error) {
	return s.Req(resource, "GET", params, nil)
}

func (s *API) Post(resource string, params map[string]string, body string) (string, error) {
	return s.Req(resource, "POST", params, []byte(body))
}

func (s *API) Req(resource string, method string, params map[string]string, body []byte) (string, error) {
	methodUrl := "/SpaceLabAPI" + resource
	date := strings.Replace(time.Now().UTC().Format(time.RFC1123), "UTC", "GMT", -1)

	reqUrl := methodUrl + "?"
	for k, v := range params {
		escapedV := url.QueryEscape(v)
		escapedK := url.QueryEscape(k)
		reqUrl += fmt.Sprintf("%s=%s&", escapedK, escapedV)
	}
	reqUrl = reqUrl[:len(reqUrl)-1] // Remove leading & or leading ? if no params

	var b io.Reader

	if len(body) > 0 {
		b = bytes.NewReader(body)
	}

	req, err := http.NewRequest(method, s.url+reqUrl, b)
	if err != nil {
		panic(err)
	}
	req.Header.Add("Date", date)

	c := http.Client{}
	resp, err := c.Do(req)
	if err != nil {
		return "", err
	}

	if resp.StatusCode != 200 {
		return "", api.SpaceError(fmt.Sprintf("invalid response: %s", resp.Status))
	}

	data, err := ioutil.ReadAll(resp.Body)
	return string(data), err
}
