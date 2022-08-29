package spacelab

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
	api     *API
	grids   map[string]Grid
	running bool
	log     slog.Instance
	bus     *pubsub.Bus
	voxels  map[string]Voxel
}

func MakeNotify(api *API) *Notify {
	return &Notify{
		api:    api,
		grids:  make(map[string]Grid),
		voxels: make(map[string]Voxel),
		log:    slog.Scope("SpaceNotify"),
		bus:    pubsub.NewBus(),
	}
}

func (n *Notify) SubscribeGridUpdate(cb func(e GridUpdate)) *pubsub.Subscription {
	return n.bus.Subscribe(gridUpdateTopic, cb)
}

func (n *Notify) SubscribeChatMessage(cb func(e Message)) *pubsub.Subscription {
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

	n.log.Info("Fetching voxels")
	voxels, err := n.api.Voxels()
	if err != nil {
		panic(err)
	}
	for _, voxel := range voxels {
		n.voxels[voxel.Name] = voxel
	}

	for n.running {
		<-checkInterval.C
		n.refresh()
	}

	n.running = false
}

func (n *Notify) GetPlanets() map[string]Voxel {
	return n.voxels
}

func (n *Notify) GetGrids() map[string]Grid {
	return n.grids
}

func (n *Notify) refresh() {
	n.updateChat()
	n.updateGrids()
}

func (n *Notify) updateChat() {
	// Update Chat
	messages, err := n.api.GetChat()
	if err != nil {
		n.log.Error("error reading chat: %s", err)
		return
	}

	// Assume in order
	for _, message := range messages {
		n.log.Info("%s (%s): %s", time.Now().Format(time.RFC3339), message.From, message.Message)
		n.bus.Publish(chatMessageTopic, message)
	}
}

func (n *Notify) updateGrids() {
	// Update Grids
	grids, err := n.api.Grids()
	if err != nil {
		n.log.Error("error reading grids: %s", err)
		return
	}

	gridsNow := make(map[string]string)
	for _, grid := range grids {
		if grid.Blocks < minNumBlocks {
			continue // Skip
		}

		notification := GridUpdate{}
		id := grid.Id
		notification.Grid = grid
		gridsNow[id] = id

		_, ok := n.grids[id]
		if !ok {
			notification.IsNew = true
			n.grids[id] = grid
			n.log.Info("Grid %q just appeared", grid.Name)
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
			n.log.Info("Grid %q just disappeared", grid.Name)
			n.bus.Publish(gridUpdateTopic, notification)
			delete(n.grids, id)
		}
	}
}
