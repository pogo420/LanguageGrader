package storagelogic

import (
	"encoding/json"
	ds "sandwich_go/src/datastructures"
	ut "sandwich_go/src/utils"

	log "github.com/sirupsen/logrus"
)

type JsonStorageLogic struct {
	file_path string
}

// Private function to get sandwich collection
func readSandwichCollection(file_path string) ds.SandwichCollection {
	data := ut.ReadFile(file_path)
	var sandwichCollection ds.SandwichCollection
	err := json.Unmarshal([]byte(data), &sandwichCollection.Sandwiches)

	if err != nil {
		log.Warn("Issue in reading json data:", err)
		return ds.SandwichCollection{}
	}
	return sandwichCollection
}

// Method to setup file path
func (jsl *JsonStorageLogic) Setup(file_path string) {
	jsl.file_path = file_path
}

// Method to get sandwich
func (jsl *JsonStorageLogic) Get_sandwich(name string) ds.Sandwich {
	swc := readSandwichCollection(jsl.file_path)

	for _, sw := range swc.Sandwiches {
		if sw.GetName() == name {
			return sw
		}
	}
	log.Warn("Not able to get key", name, " in Sandwich collection")
	return ds.Sandwich{}
}

func (jsl *JsonStorageLogic) Save_sandwich(sandwich ds.Sandwich) ut.Response {
	return 0
}

func (jsl *JsonStorageLogic) Delete_sandwich(sandwich ds.Sandwich) ut.Response {
	return 0
}
