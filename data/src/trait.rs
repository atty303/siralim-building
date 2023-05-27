use implicit_clone::unsync::IString;
use std::rc::Rc;
// use tantivy::schema::{Field, Schema, TextOptions, FAST, INDEXED, STORED, TEXT};
// use tantivy::Document;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Trait {
    pub id: i32,
    pub class: IString,
    pub family: IString,
    pub creature: IString,
    pub trait_name: IString,
    pub trait_description: Vec<IString>,
    pub material_name: IString,
    pub sources: Vec<IString>,
    pub sprite: Option<IString>,
    pub stats: Option<Stats>,
}

impl Trait {
    pub fn health(&self) -> Option<u8> {
        self.stats.as_ref().map(|x| x.health)
    }
    pub fn attack(&self) -> Option<u8> {
        self.stats.as_ref().map(|x| x.attack)
    }
    pub fn intelligence(&self) -> Option<u8> {
        self.stats.as_ref().map(|x| x.intelligence)
    }
    pub fn defense(&self) -> Option<u8> {
        self.stats.as_ref().map(|x| x.defense)
    }
    pub fn speed(&self) -> Option<u8> {
        self.stats.as_ref().map(|x| x.speed)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Stats {
    pub health: u8,
    pub attack: u8,
    pub intelligence: u8,
    pub defense: u8,
    pub speed: u8,
}

// pub struct TraitSchema {
//     schema: Schema,
// }
//
// impl TraitSchema {
//     pub fn new() -> TraitSchema {
//         let mut schema_builder = Schema::builder();
//         schema_builder.add_i64_field("id", INDEXED | STORED);
//         schema_builder.add_text_field("class", TEXT | STORED | FAST);
//         schema_builder.add_text_field("family", TEXT | STORED);
//         schema_builder.add_text_field("creature", TEXT | STORED);
//         schema_builder.add_text_field("name", TEXT | STORED);
//         schema_builder.add_text_field("description", TEXT | STORED);
//         schema_builder.add_text_field("material", TEXT | STORED);
//         schema_builder.add_text_field("sources", TextOptions::default().set_stored());
//         schema_builder.add_text_field("sprite", TextOptions::default().set_stored());
//         schema_builder.add_u64_field("health", STORED);
//         schema_builder.add_u64_field("attack", STORED);
//         schema_builder.add_u64_field("intelligence", STORED);
//         schema_builder.add_u64_field("defense", STORED);
//         schema_builder.add_u64_field("speed", STORED);
//         schema_builder.add_u64_field("total", STORED);
//
//         Self {
//             schema: schema_builder.build(),
//         }
//     }
//
//     pub fn from(schema: Schema) -> TraitSchema {
//         Self { schema }
//     }
//
//     pub fn schema(&self) -> Schema {
//         self.schema.clone()
//     }
//
//     pub fn id(&self) -> Field {
//         self.schema.get_field("id").unwrap()
//     }
//     pub fn class(&self) -> Field {
//         self.schema.get_field("class").unwrap()
//     }
//     pub fn family(&self) -> Field {
//         self.schema.get_field("family").unwrap()
//     }
//     pub fn creature(&self) -> Field {
//         self.schema.get_field("creature").unwrap()
//     }
//     pub fn name(&self) -> Field {
//         self.schema.get_field("name").unwrap()
//     }
//     pub fn description(&self) -> Field {
//         self.schema.get_field("description").unwrap()
//     }
//     pub fn material(&self) -> Field {
//         self.schema.get_field("material").unwrap()
//     }
//     pub fn sources(&self) -> Field {
//         self.schema.get_field("sources").unwrap()
//     }
//     pub fn sprite(&self) -> Field {
//         self.schema.get_field("sprite").unwrap()
//     }
//     pub fn health(&self) -> Field {
//         self.schema.get_field("health").unwrap()
//     }
//     pub fn attack(&self) -> Field {
//         self.schema.get_field("attack").unwrap()
//     }
//     pub fn intelligence(&self) -> Field {
//         self.schema.get_field("intelligence").unwrap()
//     }
//     pub fn defense(&self) -> Field {
//         self.schema.get_field("defense").unwrap()
//     }
//     pub fn speed(&self) -> Field {
//         self.schema.get_field("speed").unwrap()
//     }
//     pub fn total(&self) -> Field {
//         self.schema.get_field("total").unwrap()
//     }
//
//     fn get_text(doc: &Document, field: Field) -> IString {
//         IString::from(Rc::from(doc.get_first(field).unwrap().as_text().unwrap()))
//     }
//
//     fn get_text_all(doc: &Document, field: Field) -> Vec<IString> {
//         doc.get_all(field)
//             .map(|t| IString::from(Rc::from(t.as_text().unwrap())))
//             .collect()
//     }
//
//     pub fn to_struct(&self, doc: &Document) -> Trait {
//         let stats = if let Some(health) = doc.get_first(self.health()).map(|v| v.as_u64().unwrap())
//         {
//             Some(Stats {
//                 health: health as u8,
//                 attack: doc.get_first(self.attack()).unwrap().as_u64().unwrap() as u8,
//                 intelligence: doc
//                     .get_first(self.intelligence())
//                     .unwrap()
//                     .as_u64()
//                     .unwrap() as u8,
//                 defense: doc.get_first(self.defense()).unwrap().as_u64().unwrap() as u8,
//                 speed: doc.get_first(self.speed()).unwrap().as_u64().unwrap() as u8,
//             })
//         } else {
//             None
//         };
//         Trait {
//             id: doc.get_first(self.id()).unwrap().as_i64().unwrap() as i32,
//             class: TraitSchema::get_text(doc, self.class()),
//             family: TraitSchema::get_text(doc, self.family()),
//             creature: TraitSchema::get_text(doc, self.creature()),
//             trait_name: TraitSchema::get_text(doc, self.name()),
//             trait_description: TraitSchema::get_text_all(doc, self.description()),
//             material_name: TraitSchema::get_text(doc, self.material()),
//             sources: TraitSchema::get_text_all(doc, self.sources()),
//             sprite: doc
//                 .get_first(self.sprite())
//                 .map(|x| IString::from(x.as_text().unwrap().to_string())),
//             stats,
//         }
//     }
// }
