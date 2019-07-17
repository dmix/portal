// -----------------------------------------------------------------------
// Database
// -----------------------------------------------------------------------

use portal::Dir;
use std::collections::BTreeMap;
use std::path::Path;
use tantivy;
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, ReloadPolicy};
use tempdir::TempDir;

pub struct DB {
    schema: Schema,
    index: Index,
}

pub fn init() -> tantivy::Result<DB> {
    let path = Path::new("./db/");
    // let index_path = TempDir::new("portal_tmp_dir")?;
    let index_path = MmapDirectory::open(path).unwrap();
    // let index_path = ManagedDirectory::wrap(path);

    let mut schema_builder = Schema::builder();

    let timestamp_options = IntOptions::default()
        .set_stored()
        .set_fast(Cardinality::SingleValue);
    schema_builder.add_text_field("path", TEXT | STORED);
    // schema_builder.add_u64_field("count", FAST);
    schema_builder.add_u64_field("timestamp", timestamp_options);

    let schema = schema_builder.build();
    let index = Index::open_or_create(index_path, schema.clone())?;

    Ok(DB { schema, index })
}

pub fn seed() -> tantivy::Result<Vec<Dir>> {
    // println!("DB SEED");

    let entries = vec![
        Dir::new("/Users/dmix/dev/_rust/portal", 1557849352),
        Dir::new("/Users/dmix/dev/_elixir/issues", 1561657040),
        Dir::new("/Users/dmix/dev/_nim/karax/examples", 1549258325),
    ];

    Ok(entries)
}

pub fn add_entries(db: &DB, entries: Vec<Dir>) {
    println!("DB ADDING {} ENTRIES", entries.len());
    let mut index_writer = db.index.writer(50_000_000).unwrap();

    let path = db.schema.get_field("path").unwrap();
    let timestamp = db.schema.get_field("timestamp").unwrap();

    for e in entries {
        println!("DB ADDING {}", e.path);
        let mut entry = Document::default();
        entry.add_text(path, &e.path);
        entry.add_u64(timestamp, e.timestamp as u64);
        index_writer.add_document(entry);
    }

    index_writer.commit().expect("Error adding entries db!");
}

pub fn parseDoc(db: &DB, doc: &Document) -> Dir {
    let mut field_map = BTreeMap::new();

    for (field, field_values) in doc.get_sorted_field_values() {
        let field_name = db.schema.get_field_name(field);
        let values: Vec<Value> = field_values
            .into_iter()
            .map(FieldValue::value)
            .cloned()
            .collect();
        field_map.insert(field_name.to_string(), values);
    }

    Dir::new(
        field_map["path"][0].text().unwrap(),
        field_map["timestamp"][0].u64_value() as u32,
    )
}

pub fn query(db: &DB, query_term: &str) -> Vec<Dir> {
    // println!("DB SEARCH: {}", &query_term);

    let path = db.schema.get_field("path").unwrap();
    // let timestamp = db.schema.get_field("timestamp").unwrap();

    let reader = db
        .index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();
    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&db.index, vec![path]);
    let query = query_parser.parse_query(&query_term).unwrap();

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(50))
        .expect("could not parse query");

    // println!("DB FOUND {} RESULTS", top_docs.len());

    let mut results = Vec::new();
    for (_d, doc_address) in top_docs {
        let doc = searcher.doc(doc_address).unwrap();
        let entry = parseDoc(&db, &doc);
        results.push(entry);
    }

    results
}
