/// Determine if a plant species bears edible fruit
pub fn is_fruit_bearing_species(species: &str) -> bool {
    let species_lower = species.to_lowercase();
    
    match species_lower.as_str() {
        // Definite fruit-bearing vegetables
        "tomato" | "tomatoes" => true,
        "pepper" | "peppers" | "bell pepper" | "chili" | "jalapeño" | "jalapeno" => true,
        "cucumber" | "cucumbers" => true,
        "squash" | "zucchini" | "pumpkin" => true,
        "melon" | "watermelon" | "cantaloupe" | "honeydew" => true,
        "eggplant" | "aubergine" => true,
        
        // Legumes (pods are technically fruits)
        "bean" | "beans" | "green bean" | "snap bean" => true,
        "pea" | "peas" | "snap pea" | "snow pea" => true,
        
        // Berry-producing plants
        "strawberry" | "strawberries" => true,
        "blueberry" | "blueberries" => true,
        "raspberry" | "raspberries" => true,
        "blackberry" | "blackberries" => true,
        
        // Tree fruits (if we ever germinate these)
        "apple" | "apples" => true,
        "pear" | "pears" => true,
        "peach" | "peaches" => true,
        "cherry" | "cherries" => true,
        
        // Leafy greens and herbs (no edible fruit)
        "lettuce" | "salad" => false,
        "spinach" => false,
        "kale" => false,
        "chard" | "swiss chard" => false,
        "arugula" | "rocket" => false,
        "collard" | "collard greens" => false,
        
        // Herbs (leaves/stems harvested, not fruit)
        "basil" => false,
        "cilantro" | "coriander" => false,
        "parsley" => false,
        "oregano" => false,
        "thyme" => false,
        "rosemary" => false,
        "sage" => false,
        "mint" => false,
        "dill" => false,
        "chives" => false,
        
        // Root vegetables (no edible fruit)
        "carrot" | "carrots" => false,
        "radish" | "radishes" => false,
        "beet" | "beets" | "beetroot" => false,
        "turnip" | "turnips" => false,
        "potato" | "potatoes" => false,
        
        // Brassicas (leaves/heads harvested, not fruit)
        "broccoli" => false,
        "cauliflower" => false,
        "cabbage" => false,
        "brussels sprouts" => false,
        
        // Grains and grasses (seeds/stalks harvested, not fruit)
        "wheat" => false, // Grain harvested
        "sugar cane" | "sugarcane" | "cane sugar" => false, // Stalks harvested
        
        // Cannabis (technically has fruit but not typically "edible fruit")
        "cannabis" | "marijuana" | "hemp" => false, // Flowers are harvested, not fruit
        
        // Default: unknown species, assume no edible fruit
        _ => false,
    }
}

/// Get common name for fruit type by species
pub fn get_fruit_type(species: &str) -> Option<&'static str> {
    let species_lower = species.to_lowercase();
    
    match species_lower.as_str() {
        "tomato" | "tomatoes" => Some("tomatoes (fruits)"),
        "pepper" | "peppers" | "bell pepper" | "chili" => Some("peppers (fruits)"),
        "cucumber" | "cucumbers" => Some("cucumbers (fruits)"),
        "squash" | "zucchini" => Some("squash/zucchini (fruits)"),
        "pumpkin" => Some("pumpkins (fruits)"),
        "melon" | "watermelon" | "cantaloupe" | "honeydew" => Some("melons (fruits)"),
        "eggplant" | "aubergine" => Some("eggplant (fruit)"),
        "bean" | "beans" | "green bean" => Some("beans (pods/fruits)"),
        "pea" | "peas" | "snap pea" | "snow pea" => Some("peas (pods/fruits)"),
        "strawberry" | "strawberries" => Some("strawberries (berries)"),
        "blueberry" | "blueberries" => Some("blueberries (berries)"),
        "raspberry" | "raspberries" => Some("raspberries (berries)"),
        "blackberry" | "blackberries" => Some("blackberries (berries)"),
        "cantaloupe" | "cantelope" => Some("cantaloupe (melon fruit)"),
        "watermelon" => Some("watermelon (fruit)"),
        _ => None,
    }
}

/// Get harvested part description for non-fruit species
pub fn get_harvested_part(species: &str) -> &'static str {
    let species_lower = species.to_lowercase();
    
    if is_fruit_bearing_species(species) {
        return "fruit";
    }
    
    match species_lower.as_str() {
        // Leafy vegetables
        "lettuce" | "spinach" | "kale" | "chard" | "arugula" | "collard" => "leaves",
        
        // Herbs
        "basil" | "cilantro" | "parsley" | "oregano" | "thyme" | "rosemary" | "sage" | "mint" | "dill" => "leaves/stems",
        
        // Root vegetables
        "carrot" | "radish" | "beet" | "turnip" | "potato" => "roots",
        
        // Brassicas
        "broccoli" | "cauliflower" => "flower heads",
        "cabbage" | "brussels sprouts" => "leaves/heads",
        
        // Grains
        "wheat" => "grain/seeds",
        "sugar cane" | "sugarcane" | "cane sugar" => "stalks/juice",
        
        // Cannabis
        "cannabis" | "marijuana" | "hemp" => "flowers",
        
        // Default
        _ => "plant parts",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fruit_bearing_species() {
        assert_eq!(is_fruit_bearing_species("tomato"), true);
        assert_eq!(is_fruit_bearing_species("TOMATO"), true);
        assert_eq!(is_fruit_bearing_species("Tomatoes"), true);
        assert_eq!(is_fruit_bearing_species("pepper"), true);
        assert_eq!(is_fruit_bearing_species("cucumber"), true);
        
        assert_eq!(is_fruit_bearing_species("basil"), false);
        assert_eq!(is_fruit_bearing_species("lettuce"), false);
        assert_eq!(is_fruit_bearing_species("spinach"), false);
        assert_eq!(is_fruit_bearing_species("carrot"), false);
    }
    
    #[test]
    fn test_get_fruit_type() {
        assert_eq!(get_fruit_type("tomato"), Some("tomatoes (fruits)"));
        assert_eq!(get_fruit_type("pepper"), Some("peppers (fruits)"));
        assert_eq!(get_fruit_type("basil"), None);
        assert_eq!(get_fruit_type("lettuce"), None);
    }
    
    #[test]
    fn test_get_harvested_part() {
        assert_eq!(get_harvested_part("tomato"), "fruit");
        assert_eq!(get_harvested_part("pepper"), "fruit");
        assert_eq!(get_harvested_part("basil"), "leaves/stems");
        assert_eq!(get_harvested_part("lettuce"), "leaves");
        assert_eq!(get_harvested_part("carrot"), "roots");
        assert_eq!(get_harvested_part("broccoli"), "flower heads");
    }
}

