package main

import (
	"net/http"

	"github.com/gorilla/websocket"
	"github.com/racerxdl/spacelabview/spacelab"
	"github.com/racerxdl/spacelabview/spaceproto"
	"google.golang.org/protobuf/proto"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true
	},
} // use default options

type wsMessage struct {
	Type    string
	Content interface{}
}

func wshandler(notify *spacelab.Notify, w http.ResponseWriter, r *http.Request) {
	c, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Error("upgrade:", err)
		return
	}
	defer c.Close()
	globalInfoSub := notify.SubscribeGlobalInfo(func(e spacelab.GlobalInfo) {
		m := &spaceproto.WebsocketMessage{
			Type: "globalInfo",
			Data: &spaceproto.WebsocketMessage_GlobalInfo{
				GlobalInfo: e.ToProto(),
			},
		}

		d, _ := proto.Marshal(m)
		c.WriteMessage(websocket.BinaryMessage, d)
	})
	defer notify.Unsubscribe(globalInfoSub)

	chatSub := notify.SubscribeChatMessage(func(e spacelab.Message) {
		m := &spaceproto.WebsocketMessage{
			Type: "chat",
			Data: &spaceproto.WebsocketMessage_SpaceMessage{
				SpaceMessage: e.ToProto(),
			},
		}
		d, _ := proto.Marshal(m)
		c.WriteMessage(websocket.BinaryMessage, d)
	})
	defer notify.Unsubscribe(chatSub)

	gridSub := notify.SubscribeGridUpdate(func(e spacelab.GridUpdate) {
		m := &spaceproto.WebsocketMessage{
			Type: "gridUpdate",
			Data: &spaceproto.WebsocketMessage_GridUpdate{
				GridUpdate: e.ToProto(),
			},
		}
		d, _ := proto.Marshal(m)
		c.WriteMessage(websocket.BinaryMessage, d)
	})
	defer notify.Unsubscribe(gridSub)

	playerSub := notify.SubscribePlayerUpdate(func(e spacelab.PlayerUpdate) {
		m := &spaceproto.WebsocketMessage{
			Type: "playerUpdate",
			Data: &spaceproto.WebsocketMessage_PlayerUpdate{
				PlayerUpdate: e.ToProto(),
			},
		}
		d, _ := proto.Marshal(m)
		c.WriteMessage(websocket.BinaryMessage, d)
	})
	defer notify.Unsubscribe(playerSub)

	planets := notify.GetPlanetsProto()
	m := &spaceproto.WebsocketMessage{
		Type: "planets",
		Data: &spaceproto.WebsocketMessage_PlanetList{
			PlanetList: planets,
		},
	}
	d, _ := proto.Marshal(m)
	c.WriteMessage(websocket.BinaryMessage, d)

	grids := notify.GetGridsProto()
	m = &spaceproto.WebsocketMessage{
		Type: "grids",
		Data: &spaceproto.WebsocketMessage_GridList{
			GridList: grids,
		},
	}
	d, _ = proto.Marshal(m)
	c.WriteMessage(websocket.BinaryMessage, d)

	players := notify.GetPlayersProto()
	m = &spaceproto.WebsocketMessage{
		Type: "players",
		Data: &spaceproto.WebsocketMessage_Players{
			Players: players,
		},
	}
	d, _ = proto.Marshal(m)
	c.WriteMessage(websocket.BinaryMessage, d)

	for {
		protoMsg := &spaceproto.WebsocketMessage{}
		_, message, err := c.ReadMessage()
		if err != nil {
			log.Error("read:", err)
			break
		}

		err = proto.Unmarshal(message, protoMsg)
		if err != nil {
			log.Error("unmarshal:", err)
			continue
		}

		log.Info("recv: %s", protoMsg.Type)
		switch protoMsg.Type {
		case "requestGridBlocks":
			gridId := protoMsg.GetGridBlockRequest().GridId
			blocks := notify.GetGridBlocksProto(gridId)
			m := &spaceproto.WebsocketMessage{
				Type: "gridBlocks",
				Data: &spaceproto.WebsocketMessage_GridBlocks{
					GridBlocks: blocks,
				},
			}
			d, _ := proto.Marshal(m)
			c.WriteMessage(websocket.BinaryMessage, d)
		}
	}
}

func StartWeb(notify *spacelab.Notify) {
	fs := http.FileServer(http.Dir("./webroot"))

	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
		wshandler(notify, w, r)
	})
	http.Handle("/", fs)
	err := http.ListenAndServe(":3000", nil)
	if err != nil {
		log.Fatal(err)
	}
}
