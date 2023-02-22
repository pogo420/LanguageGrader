package tests

import (
	ds "sandwich_go/src/datastructures"
	si "sandwich_go/src/storageinterface"
	ut "sandwich_go/src/utils"
	"testing"
)

type St struct {
	file_path string
}

func (s1 *St) Setup(file_path string) {
	s1.file_path = file_path
}

func (s1 *St) Get_sandwich(name string) ds.Sandwich {
	return ds.Sandwich{}
}

func (s1 *St) Save_sandwich(sandwich ds.Sandwich) ut.Response {
	return 0
}

func (s1 *St) Delete_sandwich(sandwich ds.Sandwich) ut.Response {
	return 0
}

// Unit testcase for checking storage interface
func TestStorageInterface(t *testing.T) {
	var si_ si.StorageInterface = &St{}
	si_.Setup("oloo")

	// TODO close remaining test cases
}
