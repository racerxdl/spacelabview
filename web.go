package main

import (
	"encoding/json"
	"net/http"

	"github.com/gorilla/websocket"
	"github.com/racerxdl/spacelabview/api"
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

func wshandler(notify *api.Notify, w http.ResponseWriter, r *http.Request) {
	c, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Error("upgrade:", err)
		return
	}
	defer c.Close()
	chatSub := notify.SubscribeChatMessage(func(e api.ChatMessage) {
		m := wsMessage{Type: "chat", Content: e}
		d, _ := json.Marshal(m)
		c.WriteMessage(websocket.TextMessage, d)
	})
	defer notify.Unsubscribe(chatSub)

	gridSub := notify.SubscribeGridUpdate(func(e api.GridUpdate) {
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

func StartWeb(notify *api.Notify) {
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
