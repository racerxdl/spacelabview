package spacelab

import (
	"time"

	"github.com/quan-to/slog"
	"github.com/racerxdl/spacelabview/spaceproto"
	"github.com/simonfxr/pubsub"
)

const (
	globalUpdateTopic = "globalUpdate"
	gridUpdateTopic   = "gridUpdate"
	playerUpdateTopic = "playerUpdate"
	chatMessageTopic  = "chatMessage"
	minDeltaUpdate    = 10 // Meters
	minNumBlocks      = 20
)

type Notify struct {
	api     *API
	grids   map[string]Grid
	running bool
	log     slog.Instance
	bus     *pubsub.Bus
	voxels  map[string]Voxel
	players map[string]Player
	blocks  map[string][]GridBlock
	global  GlobalInfo
}

func MakeNotify(api *API) *Notify {
	return &Notify{
		api:     api,
		grids:   make(map[string]Grid),
		voxels:  make(map[string]Voxel),
		players: make(map[string]Player),
		blocks:  make(map[string][]GridBlock),
		log:     slog.Scope("SpaceNotify"),
		bus:     pubsub.NewBus(),
	}
}

func (n *Notify) SubscribeGridUpdate(cb func(e GridUpdate)) *pubsub.Subscription {
	return n.bus.Subscribe(gridUpdateTopic, cb)
}

func (n *Notify) SubscribePlayerUpdate(cb func(e PlayerUpdate)) *pubsub.Subscription {
	return n.bus.Subscribe(playerUpdateTopic, cb)
}

func (n *Notify) SubscribeChatMessage(cb func(e Message)) *pubsub.Subscription {
	return n.bus.Subscribe(chatMessageTopic, cb)
}
func (n *Notify) SubscribeGlobalInfo(cb func(e GlobalInfo)) *pubsub.Subscription {
	return n.bus.Subscribe(globalUpdateTopic, cb)
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

	n.log.Info("Notify Loop reached")
	for n.running {
		<-checkInterval.C
		n.refresh()
	}

	n.running = false
}

func (n *Notify) GetPlanets() map[string]Voxel {
	return n.voxels
}

func (n *Notify) GetPlanetsProto() *spaceproto.PlanetList {
	planets := make(map[string]*spaceproto.Voxel)
	for k, v := range n.voxels {
		planets[k] = v.ToProto()
	}
	return &spaceproto.PlanetList{
		Planets: planets,
	}
}

func (n *Notify) GetGrids() map[string]Grid {
	return n.grids
}

func (n *Notify) GetGridsProto() *spaceproto.GridList {
	grids := make(map[string]*spaceproto.Grid)
	for k, v := range n.grids {
		grids[k] = v.ToProto()
	}
	return &spaceproto.GridList{
		Grids: grids,
	}
}

func (n *Notify) GetPlayers() map[string]Player {
	return n.players
}

func (n *Notify) GetPlayersProto() *spaceproto.Players {
	players := make(map[string]*spaceproto.Player)
	for k, v := range n.players {
		players[k] = v.ToProto()
	}
	return &spaceproto.Players{
		Players: players,
	}
}

func (n *Notify) GetGridBlocks(gridId string) []GridBlock {
	return n.blocks[gridId]
}

func (n *Notify) GetGridBlocksProto(gridId string) *spaceproto.GridBlocks {
	var blocks = []*spaceproto.GridBlock{}
	for _, v := range n.blocks[gridId] {
		blocks = append(blocks, v.ToProto())
	}
	return &spaceproto.GridBlocks{
		Blocks: blocks,
	}
}

func (n *Notify) refresh() {
	n.updateGlobal()
	n.updateChat()
	n.updateGrids()
	n.updatePlayers()
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
func (n *Notify) updateGlobal() {
	ginfo, err := n.api.GlobalInfo()
	if err != nil {
		n.log.Error("error reading global info: %s", err)
		return
	}
	if ginfo.ChangedSince(n.global) {
		n.global = ginfo
		n.bus.Publish(globalUpdateTopic, n.global)
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
		if grid.Blocks < minNumBlocks && !grid.IsStatic {
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

		if !ok || grid.LastBlocksUpdate-g.LastBlocksUpdate > 30 {
			// n.log.Debug("Grid %q blocks updated", grid.Name)
			blocks, err := n.api.GridBlocks(id)
			if err != nil {
				n.log.Error("error reading grid blocks: %s", err)
			} else {
				g.LastBlocksUpdate = grid.LastBlocksUpdate
				n.blocks[id] = blocks
				n.grids[id] = g // Struct, so this is needed
			}
		}

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

func (n *Notify) updatePlayers() {
	// Update Grids
	players, err := n.api.Players()
	if err != nil {
		n.log.Error("error reading players: %s", err)
		return
	}

	playersNow := make(map[string]string)
	for _, player := range players {
		if !player.IsOnline {
			continue // Skip
		}
		notification := PlayerUpdate{}
		id := player.Id
		notification.Player = player
		playersNow[id] = id

		_, ok := n.players[id]
		if !ok {
			notification.IsNew = true
			n.players[id] = player
			n.log.Info("Player %q just appeared", player.Name)
		}

		g := n.players[id]

		if g.DistanceTo(player) > minDeltaUpdate {
			n.players[id] = player
			n.bus.Publish(playerUpdateTopic, notification)
		}
	}

	for id, player := range n.players {
		_, ok := playersNow[id]
		if !ok {
			notification := PlayerUpdate{}
			notification.IsDeleted = true
			notification.Player = player
			n.log.Info("Player %q just disappeared", player.Name)
			n.bus.Publish(playerUpdateTopic, notification)
			delete(n.players, id)
		}
	}
}
