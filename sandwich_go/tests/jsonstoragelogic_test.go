package tests

import (
	"sandwich_go/src/storageinterface"
	"sandwich_go/src/storagelogic"
	"testing"
)

// Unit test case for checking valid case of GetSandwich
func TestJsonStorageGetSandwichValid(t *testing.T) {
	var sl_ storageinterface.StorageInterface = &storagelogic.JsonStorageLogic{}

	sl_.Setup("test_data/valid.json")
	sw := sl_.Get_sandwich("dummy_sand_2")

	if sw.GetName() != "dummy_sand_2" {
		t.Fatal("Issue in get sandwich logic, sandwich name", sw.GetName())
	} else if sw.GetRecipe() != "Boom Boom 2" {
		t.Fatal("Issue in get sandwich logic, sandwich recipe", sw.GetRecipe())
	}
}
