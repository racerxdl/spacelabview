package main

import (
	"fmt"

	"github.com/racerxdl/spacelabview/spacelab"
)

func main() {
	LoadConfig()

	s, err := spacelab.MakeAPI(config.URL)
	if err != nil {
		panic(err)
	}

	grids, err := s.Grids()
	if err != nil {
		panic(err)
	}
	gridGroups := map[int]int{}

	for _, grid := range grids {
		if _, ok := gridGroups[grid.RelGroupId]; !ok {
			gridGroups[grid.RelGroupId] = 0
		}
		gridGroups[grid.RelGroupId]++
	}

	_, _ = s.SendMessage("Cleaning grids < 20 blocks and unowned")
	deleted := 0
	gridsToDelete := make([]spacelab.Grid, 0)
	for _, grid := range grids {
		if grid.RelGroupCount == 1 && grid.Blocks < 20 &&
			grid.Owner == "Thumbs" ||
			grid.Owner == "Space Pirates" ||
			grid.Owner == "Raider Commander" ||
			grid.Owner == "NONE" {
			fmt.Println(grid)
			gridsToDelete = append(gridsToDelete, grid)
			deleted++
		} else if grid.IsPowered { // Powered grids criteria
			if grid.Owner == "Thumbs" ||
				grid.Owner == "Space Pirates" ||
				grid.Owner == "Raider Commander" {
				fmt.Println(grid)
				gridsToDelete = append(gridsToDelete, grid)
				deleted++
			} else if grid.IsStatic && grid.Blocks < 3 {
				fmt.Println(grid)
				gridsToDelete = append(gridsToDelete, grid)
				deleted++
			}
		} else if grid.Blocks < 20 {
			fmt.Println(grid)
			gridsToDelete = append(gridsToDelete, grid)
			deleted++
		}

		// if (grid.Blocks < 20 && grid.Owner == "NONE" && !grid.IsPowered) ||
		// 	grid.Owner == "Thumbs" ||
		// 	grid.Owner == "Space Pirates" ||
		// 	grid.Owner == "Raider Commander" {
		// 	fmt.Println(grid)
		// 	gridsToDelete = append(gridsToDelete, grid)
		// 	deleted++
		// }
	}
	if len(gridsToDelete) > 0 {
		res, err := s.DeleteEntity(gridsToDelete)
		if err != nil {
			panic(err)
		}
		fmt.Println(res)
	}
	_, _ = s.SendMessage(fmt.Sprintf("Done! Deleted %d grids!", deleted))

	notify := spacelab.MakeNotify(s)
	notify.Start()
	defer notify.Stop()

	StartWeb(notify)
}
