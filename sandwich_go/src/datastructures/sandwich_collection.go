package datastructures

//Data structure - for wrapping collection of sandwiches
type SandwichCollection struct {
	sandwiches []Sandwich
}

//Public method for getting collection of sandwich objects
func (sw *SandwichCollection) GetSandwiches() []Sandwich {
	return (*sw).sandwiches
}

//Public method for setting collection of sandwich objects
func (sw *SandwichCollection) SetSandwiches(sandwiches []Sandwich) {
	(*sw).sandwiches = sandwiches
}

//Public method for add a new sandwich object
func (sw *SandwichCollection) AddSandwich(sandwich Sandwich) {
	if (*sw).isEmpty() {
		(*sw).sandwiches = make([]Sandwich, 0) // if empty create a new
	} else {
		(*sw).sandwiches = append((*sw).sandwiches, sandwich)
	}
}

//private method for getting index
func (sw *SandwichCollection) getIndex(name string) int {
	for i, v := range (*sw).sandwiches {
		if v.name == name {
			return i
		}
	}
	return -1
}

//private method for checking if collection is empty or not
func (sw *SandwichCollection) isEmpty() bool {
	if (*sw).sandwiches == nil || len((*sw).sandwiches) <= 0 {
		return true
	} else {
		return false
	}
}

// Public method for deleting sandwich from collection
// Returns >= 0 for valid delete
func (sw *SandwichCollection) DeleteSandwich(name string) int {
	// getting index from collection of sandwich
	index := (*sw).getIndex(name)
	if index < 0 {
		return index
	}
	(*sw).sandwiches = append((*sw).sandwiches[:index], (*sw).sandwiches[index+1:]...)
	return index
}
