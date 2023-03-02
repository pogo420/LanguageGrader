package datastructures

//Data structure - for wrapping collection of sandwiches
type SandwichCollection struct {
	Sandwiches []Sandwich
}

//Public method for getting collection of sandwich objects
func (sw *SandwichCollection) GetSandwiches() []Sandwich {
	return (*sw).Sandwiches
}

//Public method for setting collection of sandwich objects
func (sw *SandwichCollection) SetSandwiches(sandwiches []Sandwich) {
	(*sw).Sandwiches = sandwiches
}

//Public method for add a new sandwich object
func (sw *SandwichCollection) AddSandwich(sandwich Sandwich) {
	if (*sw).isEmpty() {
		(*sw).Sandwiches = make([]Sandwich, 0) // if empty create a new
	} else {
		(*sw).Sandwiches = append((*sw).Sandwiches, sandwich)
	}
}

//private method for getting index
func (sw *SandwichCollection) getIndex(name string) int {
	for i, v := range (*sw).Sandwiches {
		if v.Name == name {
			return i
		}
	}
	return -1
}

//private method for checking if collection is empty or not
func (sw *SandwichCollection) isEmpty() bool {
	if (*sw).Sandwiches == nil || len((*sw).Sandwiches) <= 0 {
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
	(*sw).Sandwiches = append((*sw).Sandwiches[:index], (*sw).Sandwiches[index+1:]...)
	return index
}
