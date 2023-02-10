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
	s2.SetName("name1")
	s2.SetRecipe("recipe1")

	s_ := []ds.Sandwich{(*s1), (*s2)}
	sws.SetSandwiches(s_)
	// TODO close on the test case
}
