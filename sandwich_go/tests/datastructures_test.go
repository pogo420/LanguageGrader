package tests

import (
	ds "sandwich_go/src/datastructures"
	"testing"
)

// Unit testcase for Sandwich data structure
func TestSandwichDs(t *testing.T) {
	sw := &ds.Sandwich{}
	(*sw).SetName("sandwich 1")
	(*sw).SetRecipe("recipe 1")

	if (*sw).GetName() != "sandwich 1" {
		t.Fatal("Issue with sandwich data structure")
	}
	if (*sw).GetRecipe() != "recipe 1" {
		t.Fatal("Issue with sandwich data structure")
	}
}

// Unit testcase for SandwichCollection data structure
func TestSandwichCollectionDs(t *testing.T) {

	sws := &ds.SandwichCollection{}

	s1 := &ds.Sandwich{}
	s1.SetName("name1")
	s1.SetRecipe("recipe1")

	s2 := &ds.Sandwich{}
	s2.SetName("name2")
	s2.SetRecipe("recipe2")

	s_ := []ds.Sandwich{(*s1), (*s2)}

	//GetSandwiches is tested by default in every cases
	sws.SetSandwiches(s_)
	for i, sw := range sws.GetSandwiches() {
		if i == 0 && sw.GetName() != "name1" {
			t.Fatal("Issue in sandwich collection SetSandwiches")
		} else if i == 1 && sw.GetName() != "name2" {
			t.Fatal("Issue in sandwich collection SetSandwiches")
		}
	}

	sws.DeleteSandwich("name1")
	if len((*sws).GetSandwiches()) != 1 {
		t.Fatal("Issue in sandwich collection DeleteSandwich")
	}

	sws.AddSandwich(*s2)
	if len((*sws).GetSandwiches()) != 2 {
		t.Fatal("Issue in sandwich collection AddSandwich")
	}
}
