package main

import (
	"fmt"
	"strings"
	"time"

	"github.com/racerxdl/spacelabview/spacelab"
)

var tagsToErase = []string{
	"NONE",
	"SPRT",
	"ASSERT",
	"IMBER",
}

func shouldEraseTag(tag string) bool {
	for _, v := range tagsToErase {
		if strings.EqualFold(tag, v) {
			return true
		}
	}
	return false
}

func cleanupGrids(s *spacelab.API) {
	grids, err := s.GridsV2()
	if err != nil {
		panic(err)
	}

	_, _ = s.SendMessage("Cleaning grids")
	deleted := 0
	gridsToDelete := make([]spacelab.Grid, 0)

	for _, gridGroup := range grids {
		gridGroup.Tag = strings.ToUpper(gridGroup.Tag)

		if shouldEraseTag(gridGroup.Tag) && gridGroup.Blocks < 20 {
			gridsToDelete = append(gridsToDelete, gridGroup.Grids...)
		}
	}

	// for _, grid := range grids {
	// 	if grid.RelGroupCount == 1 && grid.Blocks < 20 &&
	// 		grid.Owner == "Thumbs" ||
	// 		grid.Owner == "Space Pirates" ||
	// 		grid.Owner == "Raider Commander" ||
	// 		grid.Owner == "NONE" {
	// 		fmt.Println(grid)
	// 		gridsToDelete = append(gridsToDelete, grid)
	// 		deleted++
	// 	} else if grid.IsPowered { // Powered grids criteria
	// 		if grid.Owner == "Thumbs" ||
	// 			grid.Owner == "Space Pirates" ||
	// 			grid.Owner == "Raider Commander" {
	// 			fmt.Println(grid)
	// 			gridsToDelete = append(gridsToDelete, grid)
	// 			deleted++
	// 		} else if grid.IsStatic && grid.Blocks < 3 {
	// 			fmt.Println(grid)
	// 			gridsToDelete = append(gridsToDelete, grid)
	// 			deleted++
	// 		}
	// 	} else if grid.Blocks < 20 {
	// 		fmt.Println(grid)
	// 		gridsToDelete = append(gridsToDelete, grid)
	// 		deleted++
	// 	}
	// }
	if len(gridsToDelete) > 0 {
		res, err := s.DeleteEntity(gridsToDelete)
		if err != nil {
			panic(err)
		}
		fmt.Println(res)
	}
	_, _ = s.SendMessage(fmt.Sprintf("Done! Deleted %d grids!", deleted))
}

func main() {
	LoadConfig()

	s, err := spacelab.MakeAPI(config.URL)
	if err != nil {
		panic(err)
	}

	notify := spacelab.MakeNotify(s)
	notify.Start()
	defer notify.Stop()

	cleanupGrids(s)

	t := time.NewTicker(time.Minute * 60)

	go func() {
		for {
			<-t.C
			cleanupGrids(s)
		}
	}()

	StartWeb(notify)
}
