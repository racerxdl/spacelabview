package api

import (
	"encoding/json"
	"fmt"
	"strconv"
	"time"
)

const sessionChat = "/v1/session/chat"

const ticksPerMillisecond = uint64(10000)
const ticksSinceYearOne = uint64(637329207590000000)

type ChatMessage struct {
	DisplayName string
	SteamID     uint64
	Content     string
	Timestamp   string
}

type sessionChatResponseData struct {
	Messages []ChatMessage
}

type sessionChatResponse struct {
	Data sessionChatResponseData `json:"data"`
	Meta Meta                    `json:"meta"`
}

func (c ChatMessage) GetTime() time.Time {
	// This is wrong lol
	v, _ := strconv.ParseUint(c.Timestamp, 10, 64)
	v -= ticksSinceYearOne
	v /= ticksPerMillisecond
	return time.Unix(int64(v), 0)
}

func (s *SpaceAPI) SendMessage(message string) (ChatMessage, error) {
	data := fmt.Sprintf("%q", message)
	resp, err := s.Post(sessionChat, nil, data)
	msg := ChatMessage{}
	_ = json.Unmarshal([]byte(resp), &msg)
	return msg, err
}

func (s *SpaceAPI) GetChat(timestamp string, count int) ([]ChatMessage, error) {
	params := make(map[string]string)

	if timestamp != "" {
		params["Date"] = timestamp
	}

	params["MessageCount"] = fmt.Sprintf("%d", count)

	res, err := s.Get(sessionChat, params)
	if err != nil {
		return nil, err
	}

	g := sessionChatResponse{}
	err = json.Unmarshal([]byte(res), &g)
	return g.Data.Messages, err
}
