package spacelab

import "encoding/json"

const (
	interactEndpoint             = "/interact"
	interactDeleteEntityEndpoint = interactEndpoint + "/deleteEntity"
	interactSendMessageEndpoint  = interactEndpoint + "/sendmessage"
	interactChatEndpoint         = interactEndpoint + "/chat"
)

func (s *API) DeleteEntity(grids []Grid) (string, error) {
	body, err := json.Marshal(grids)
	if err != nil {
		return "", err
	}

	return s.Post(interactDeleteEntityEndpoint, nil, string(body))
}

func (s *API) SendMessage(message string) (string, error) {
	msg := Message{
		From:    "SpaceLab",
		Message: message,
	}
	body, err := json.Marshal(msg)
	if err != nil {
		return "", err
	}

	return s.Post(interactSendMessageEndpoint, nil, string(body))
}

func (s *API) GetChat() ([]Message, error) {
	res, err := s.Get(interactChatEndpoint, nil)
	if err != nil {
		return nil, err
	}

	g := []Message{}
	err = json.Unmarshal([]byte(res), &g)
	return g, err
}
