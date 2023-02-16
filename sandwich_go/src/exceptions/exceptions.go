package exceptions

import "fmt"

type DuplicateSandwichException struct {
	Description string
}

func (exp *DuplicateSandwichException) Error() string {
	return fmt.Sprintf("Duplicate sandwich data: %s", exp.Description)
}

type InvalidSandwichData struct {
	Description string
}

func (exp *InvalidSandwichData) Error() string {
	return fmt.Sprintf("Invalid sandwich data: %s", exp.Description)
}

type SandwichNotFoundException struct {
	Description string
}

func (exp *SandwichNotFoundException) Error() string {
	return fmt.Sprintf("Sandwich data not found: %s", exp.Description)
}

type PersistanceFileException struct {
	Description string
}

func (exp *PersistanceFileException) Error() string {
	return fmt.Sprintf("Issue in persistence of sandwich data: %s", exp.Description)
}
