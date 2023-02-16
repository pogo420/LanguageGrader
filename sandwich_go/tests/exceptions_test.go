package tests

import (
	"errors"
	exp "sandwich_go/src/exceptions"
	"testing"
)

// helper functions
func get_exp(flag int) (int, error) {
	if flag == 0 {
		return 0, nil
	} else if flag == 1 {
		return -1, &exp.DuplicateSandwichException{Description: "duplicate sandwich"}
	} else if flag == 2 {
		return -1, &exp.InvalidSandwichData{Description: "invalid data for sandwich"}
	} else if flag == 3 {
		return -1, &exp.SandwichNotFoundException{Description: "sandwich not found"}
	} else if flag == 4 {
		return -1, &exp.PersistanceFileException{Description: "sandwich persistance issue"}
	} else {
		return -1, nil
	}
}

// unit test cases to check all exception types
func TestException(t *testing.T) {

	_, err := get_exp(1)
	var e0 *exp.DuplicateSandwichException
	if !errors.As(err, &e0) {
		t.Fatal("Issue in exception")
	}

	_, err = get_exp(2)
	var e1 *exp.InvalidSandwichData
	if !errors.As(err, &e1) {
		t.Fatal("Issue in exception")
	}

	_, err = get_exp(3)
	var e2 *exp.SandwichNotFoundException
	if !errors.As(err, &e2) {
		t.Fatal("Issue in exception")
	}

	_, err = get_exp(4)
	var e3 *exp.PersistanceFileException
	if !errors.As(err, &e3) {
		t.Fatal("Issue in exception")
	}
}
