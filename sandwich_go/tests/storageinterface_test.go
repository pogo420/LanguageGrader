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

// Unit testcase for checking sanity of storage interface
func TestStorageInterface(t *testing.T) {
	var si_ si.StorageInterface = &St{}

	si_.Setup("oloo")

	if si_.Save_sandwich(ds.Sandwich{}) != 0 {
		t.Fatal("Issue with Save_sandwich logic")
	} else if si_.Delete_sandwich(ds.Sandwich{}) != 0 {
		t.Fatal("Issue with Delete_sandwich logic")
	}

	var sw = si_.Get_sandwich("")
	if sw.GetName() != "" {
		t.Fatal("Issue with get sandwich logic")
	}
}
