package main

import (
	"encoding/json"
	"net/http"

	"github.com/gorilla/websocket"
	"github.com/racerxdl/spacelabview/spacelab"
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
		m := wsMessage{Type: "globalInfo", Content: e}
		d, _ := json.Marshal(m)
		c.WriteMessage(websocket.TextMessage, d)
	})
	defer notify.Unsubscribe(globalInfoSub)

	chatSub := notify.SubscribeChatMessage(func(e spacelab.Message) {
		m := wsMessage{Type: "chat", Content: e}
		d, _ := json.Marshal(m)
		c.WriteMessage(websocket.TextMessage, d)
	})
	defer notify.Unsubscribe(chatSub)

	gridSub := notify.SubscribeGridUpdate(func(e spacelab.GridUpdate) {
		m := wsMessage{Type: "gridUpdate", Content: e}
		d, _ := json.Marshal(m)
		c.WriteMessage(websocket.TextMessage, d)
	})
	defer notify.Unsubscribe(gridSub)

	planets := notify.GetPlanets()
	m := wsMessage{Type: "planets", Content: planets}
	d, _ := json.Marshal(m)
	c.WriteMessage(websocket.TextMessage, d)

	grids := notify.GetGrids()
	m = wsMessage{Type: "grids", Content: grids}
	d, _ = json.Marshal(m)
	c.WriteMessage(websocket.TextMessage, d)

	for {
		_, message, err := c.ReadMessage()
		if err != nil {
			log.Error("read:", err)
			break
		}
		log.Info("recv: %s", message)
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
