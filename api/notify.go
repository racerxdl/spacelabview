package api

import (
	"time"

	"github.com/quan-to/slog"
	"github.com/simonfxr/pubsub"
)

const (
	gridUpdateTopic  = "gridUpdate"
	chatMessageTopic = "chatMessage"
	minDeltaUpdate   = 10 // Meters
	minNumBlocks     = 20
)

type Notify struct {
	api               *SpaceAPI
	grids             map[uint64]Grid
	running           bool
	log               slog.Instance
	bus               *pubsub.Bus
	lastMessage       string
	lastChatTimestamp string
	planets           map[string]Planet
}

func MakeNotify(api *SpaceAPI) *Notify {
	return &Notify{
		api:     api,
		grids:   make(map[uint64]Grid),
		planets: make(map[string]Planet),
		log:     slog.Scope("SpaceNotify"),
		bus:     pubsub.NewBus(),
	}
}

func (n *Notify) SubscribeGridUpdate(cb func(e GridUpdate)) *pubsub.Subscription {
	return n.bus.Subscribe(gridUpdateTopic, cb)
}

func (n *Notify) SubscribeChatMessage(cb func(e ChatMessage)) *pubsub.Subscription {
	return n.bus.Subscribe(chatMessageTopic, cb)
}

func (n *Notify) Unsubscribe(subscription *pubsub.Subscription) {
	n.bus.Unsubscribe(subscription)
}

func (n *Notify) Start() {
	if n.running {
		return
	}

	n.running = true
	go n.loop()
}

func (n *Notify) Stop() {
	n.running = false
	// TODO: Wait stop
}

func (n *Notify) loop() {
	n.log.Info("Started")

	checkInterval := time.NewTicker(time.Second)
	defer checkInterval.Stop()

	n.log.Info("Fetching planets")
	planets, err := n.api.SessionPlanets()
	if err != nil {
		panic(err)
	}
	for _, planet := range planets {
		n.planets[planet.DisplayName] = planet
	}

	for n.running {
		<-checkInterval.C
		n.refresh()
	}

	n.running = false
}

func (n *Notify) GetPlanets() map[string]Planet {
	return n.planets
}

func (n *Notify) GetGrids() map[uint64]Grid {
	return n.grids
}

func (n *Notify) refresh() {
	n.updateChat()
	n.updateGrids()
}

func (n *Notify) updateChat() {
	// Update Chat
	messages, err := n.api.GetChat(n.lastChatTimestamp, 100)
	if err != nil {
		n.log.Error("error reading chat: %s", err)
		return
	}

	// Assume in order
	for _, message := range messages {
		if message.Content != n.lastMessage {
			n.log.Info("%s (%s): %s", message.GetTime().Format(time.RFC3339), message.DisplayName, message.Content)
			n.bus.Publish(chatMessageTopic, message)
		}
	}

	if len(messages) > 0 {
		lastMsg := messages[len(messages)-1]
		n.lastChatTimestamp = lastMsg.Timestamp
		n.lastMessage = lastMsg.Content
	}
}

func (n *Notify) updateGrids() {
	// Update Grids
	grids, err := n.api.SessionGrids()
	if err != nil {
		n.log.Error("error reading grids: %s", err)
		return
	}

	gridsNow := make(map[uint64]uint64)
	for _, grid := range grids {
		if grid.BlocksCount < minNumBlocks {
			continue // Skip
		}

		notification := GridUpdate{}
		id := grid.EntityId
		notification.Grid = grid
		gridsNow[id] = id

		_, ok := n.grids[id]
		if !ok {
			notification.IsNew = true
			n.grids[id] = grid
			n.log.Info("Grid %q just appeared", grid.DisplayName)
		}

		g := n.grids[id]

		if g.DistanceToGrid(grid) > minDeltaUpdate {
			n.grids[id] = grid
			n.bus.Publish(gridUpdateTopic, notification)
		}
	}

	for id, grid := range n.grids {
		_, ok := gridsNow[id]
		if !ok {
			notification := GridUpdate{}
			notification.IsDeleted = true
			notification.Grid = grid
			n.log.Info("Grid %q just disappeared", grid.DisplayName)
			n.bus.Publish(gridUpdateTopic, notification)
			delete(n.grids, id)
		}
	}
}
