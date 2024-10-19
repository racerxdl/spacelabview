package spacelab

import "github.com/racerxdl/spacelabview/spaceproto"

type Message struct {
	From    string
	Message string
}

func (m Message) ToProto() *spaceproto.SpaceMessage {
	return &spaceproto.SpaceMessage{
		From:    m.From,
		Message: m.Message,
	}
}
