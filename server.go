package main

import (
	"fmt"

	"github.com/racerxdl/spacelabview/api"
)

func main() {
	LoadConfig()

	s, err := api.MakeAPI(config.URL, config.Key)
	if err != nil {
		panic(err)
	}

	grids, err := s.SessionGrids()
	if err != nil {
		panic(err)
	}
	_, _ = s.SendMessage("Cleaning grids < 20 blocks and unowned")
	deleted := 0
	for _, grid := range grids {
		if grid.BlocksCount < 20 ||
			grid.OwnerDisplayName == "" ||
			grid.OwnerDisplayName == "Thumbs" ||
			grid.OwnerDisplayName == "Space Pirates" ||
			grid.OwnerDisplayName == "Raider Commander" {
			fmt.Println(grid)
			err = s.DeleteGrid(grid.EntityId)
			if err != nil {
				panic(err)
			}
			deleted++
		}
	}
	_, _ = s.SendMessage(fmt.Sprintf("Done! Deleted %d grids!", deleted))

	notify := api.MakeNotify(s)
	notify.Start()
	defer notify.Stop()

	StartWeb(notify)
}
