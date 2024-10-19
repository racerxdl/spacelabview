package main

import (
	"fmt"
	"strings"
	"time"

	"github.com/racerxdl/spacelabview/spacelab"
)

var tagsToErase = []string{
	"SPRT",
	"ASSERT",
	"IMBER",
	"GC",
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
	grids, err := s.Grids()
	if err != nil {
		panic(err)
	}

	_, _ = s.SendMessage("Cleaning grids")
	gridsToDelete := make([]spacelab.Grid, 0)

	for _, gridGroup := range grids {
		gridGroup.FactionTag = strings.ToUpper(gridGroup.FactionTag)

		// if shouldEraseTag(gridGroup.FactionTag) && gridGroup.Blocks < 20 {
		// 	gridsToDelete = append(gridsToDelete, gridGroup)
		// }

		if strings.Contains(gridGroup.Name, "(NPC-AAW)Gunship-") ||
			strings.Contains(gridGroup.Name, "(NPC-AAW)Drone-") ||
			strings.Contains(gridGroup.Name, "(NPC-AAW)Tetrach-") ||
			strings.Contains(gridGroup.Name, "(SPRT)") {
			gridsToDelete = append(gridsToDelete, gridGroup)
		}
	}

	if len(gridsToDelete) > 0 {
		res, err := s.DeleteEntity(gridsToDelete)
		if err != nil {
			panic(err)
		}
		fmt.Println(res)
	}
	_, _ = s.SendMessage(fmt.Sprintf("Done! Deleted %d grids!", len(gridsToDelete)))
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

	// cleanupGrids(s)

	t := time.NewTicker(time.Minute * 60)

	go func() {
		for {
			<-t.C
			// cleanupGrids(s)
		}
	}()

	StartWeb(notify)
}
