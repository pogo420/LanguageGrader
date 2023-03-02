package datastructures

// Data structure - for wrapping the sandwich data.
type Sandwich struct {
	Name   string
	Recipe string
}

// Public method to get sandwich Name
func (sw *Sandwich) GetName() string {
	return (*sw).Name
}

// Public method to get sandwich Recipe
func (sw *Sandwich) GetRecipe() string {
	return (*sw).Recipe
}

// Public method to set sandwich Name
func (sw *Sandwich) SetName(Name string) {
	(*sw).Name = Name
}

// Public method to set sandwich Recipe
func (sw *Sandwich) SetRecipe(Recipe string) {
	(*sw).Recipe = Recipe
}
