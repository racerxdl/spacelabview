package main

import (
	"os"

	"github.com/BurntSushi/toml"
	"github.com/mewkiz/pkg/osutil"
	"github.com/quan-to/slog"
)

const configFile = "agaris.toml"

var log = slog.Scope("AgarisProj")

type ConfigData struct {
	Key string
	URL string
}

var config ConfigData

func LoadConfig() {
	cfg := configFile
	log.Info("Loading config %s", cfg)
	if !osutil.Exists(cfg) {
		log.Error("Config file %s does not exists.", cfg)
		os.Exit(1)
	}

	_, err := toml.DecodeFile(cfg, &config)
	if err != nil {
		log.Error("Error decoding file %s: %s", cfg, err)
		os.Exit(1)
	}
}

func SaveConfig() {
	cfg := configFile
	log.Info("Saving config %s", cfg)
	f, err := os.Create(cfg)
	if err != nil {
		log.Fatal("Error opening %s: %s", cfg, err)
	}
	e := toml.NewEncoder(f)
	err = e.Encode(&config)
	if err != nil {
		log.Fatal("Error saving data to %s: %s", cfg, err)
	}
}
