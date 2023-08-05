#[derive(Debug, Clone)]
pub struct Realm {
    pub name: &'static str,
    pub god: &'static str,
}

impl Realm {
    pub fn new(name: &'static str, god: &'static str) -> Realm {
        Realm { name, god }
    }

    pub fn load() -> Vec<Realm> {
        vec![
            Realm::new("Forgotten Lab", "4080"),
            Realm::new("Unsullied Meadows", "Aeolian"),
            Realm::new("Damarel", "Alexandria"),
            Realm::new("Forbidden Depths", "Anneltha"),
            Realm::new("Blood Grove", "Apocranox"),
            Realm::new("Land of Breath & Balance", "Ariamaki"),
            Realm::new("Temple of Lies", "Aurum"),
            Realm::new("Frostbite Cavern", "Azural"),
            Realm::new("Path of the Damned", "Erebyss"),
            Realm::new("Where the Dead Ships Dwell", "Friden"),
            Realm::new("Overgrown Temple", "Genaros"),
            Realm::new("Kingdom of Heretics", "Gonfurian"),
            Realm::new("Faraway Enclave", "Lister"),
            Realm::new("The Swamplands", "Meraxis"),
            Realm::new("Titan's Wound", "Mortem"),
            Realm::new("Astral Gallery", "Muse"),
            Realm::new("Sanctum Imbra", "Perdition"),
            Realm::new("Gambler's Hive", "Reclusa"),
            Realm::new("Arachnid Nest", "Regalis"),
            Realm::new("Fae Lands", "Shallan"),
            Realm::new("Azure Dream", "Surathli"),
            Realm::new("Amalgam Gardens", "T'mere M'rgo"),
            Realm::new("Torture Chamber", "Tartarith"),
            Realm::new("Bastion of the Void", "Tenebris"),
            Realm::new("Cutthroat Jungle", "Torun"),
            Realm::new("Caustic Reactor", "Venedon"),
            Realm::new("Eternity's End", "Vertraag"),
            Realm::new("Great Pandemonium", "Vulcanar"),
            Realm::new("The Barrens", "Yseros"),
            Realm::new("Refuge of the Magi", "Zonte"),
        ]
    }
}
