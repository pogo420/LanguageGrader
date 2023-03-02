package utils

import (
	"io/ioutil"

	log "github.com/sirupsen/logrus"
)

// enum representation
type Response int

const (
	SUCCESS Response = 0
	FAILURE Response = 1
	UNKNOWN Response = 2
)

// Utility function to read a file, returns empty string in case of issues
func ReadFile(file_path string) string {
	data, err := ioutil.ReadFile(file_path)

	if err != nil {
		log.Warn("Issue in reading file, returning empty data")
		return ""
	}
	return string(data)
}
