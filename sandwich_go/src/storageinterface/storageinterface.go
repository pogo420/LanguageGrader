package storageinterface

import (
	ds "sandwich_go/src/datastructures"
	ut "sandwich_go/src/utils"
)

type StorageInterface interface {
	Setup(file_path string) // to be used for setting up struct
	Get_sandwich(name string) ds.Sandwich
	Save_sandwich(sandwich ds.Sandwich) ut.Response
	Delete_sandwich(sandwich ds.Sandwich) ut.Response
}
