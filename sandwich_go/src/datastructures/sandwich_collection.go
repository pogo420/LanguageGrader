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
	(*sw).sandwiches = append((*sw).sandwiches, sandwich)
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
