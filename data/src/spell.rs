use implicit_clone::unsync::IString;
use std::rc::Rc;
// use tantivy::schema::{Field, Schema, TextOptions, FAST, INDEXED, STORED, TEXT};
// use tantivy::Document;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Spell {
    pub id: i16,
    pub name: IString,
    pub class: IString,
    pub charges: u8,
    pub source: IString,
    pub description: Vec<IString>,
}

// pub struct SpellSchema {
//     schema: Schema,
// }
//
// impl SpellSchema {
//     pub fn new() -> SpellSchema {
//         let mut schema_builder = Schema::builder();
//         schema_builder.add_i64_field("id", INDEXED | STORED);
//         schema_builder.add_text_field("name", TEXT | STORED);
//         schema_builder.add_text_field("class", TEXT | STORED | FAST);
//         schema_builder.add_u64_field("charges", STORED);
//         schema_builder.add_text_field("source", TextOptions::default().set_stored());
//         schema_builder.add_text_field("description", TEXT | STORED);
//
//         Self {
//             schema: schema_builder.build(),
//         }
//     }
//
//     pub fn from(schema: Schema) -> SpellSchema {
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
//     pub fn name(&self) -> Field {
//         self.schema.get_field("name").unwrap()
//     }
//     pub fn class(&self) -> Field {
//         self.schema.get_field("class").unwrap()
//     }
//     pub fn charges(&self) -> Field {
//         self.schema.get_field("charges").unwrap()
//     }
//     pub fn description(&self) -> Field {
//         self.schema.get_field("description").unwrap()
//     }
//     pub fn source(&self) -> Field {
//         self.schema.get_field("source").unwrap()
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
//     pub fn to_struct(&self, doc: &Document) -> Spell {
//         Spell {
//             id: doc.get_first(self.id()).unwrap().as_i64().unwrap() as i16,
//             name: SpellSchema::get_text(doc, self.name()),
//             class: SpellSchema::get_text(doc, self.class()),
//             charges: doc.get_first(self.charges()).unwrap().as_u64().unwrap() as u8,
//             source: SpellSchema::get_text(doc, self.source()),
//             description: SpellSchema::get_text_all(doc, self.description()),
//         }
//     }
// }
