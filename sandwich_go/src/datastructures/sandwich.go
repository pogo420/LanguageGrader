package datastructures

// Data structure - for wrapping the sandwich data.
type Sandwich struct {
	name   string
	recipe string
}

// Public method to get sandwich name
func (sw *Sandwich) GetName() string {
	return (*sw).name
}

// Public method to get sandwich recipe
func (sw *Sandwich) GetRecipe() string {
	return (*sw).recipe
}

// Public method to set sandwich name
func (sw *Sandwich) SetName(name string) {
	(*sw).name = name
}

// Public method to set sandwich recipe
func (sw *Sandwich) SetRecipe(recipe string) {
	(*sw).recipe = recipe
}
