// -----------------------------------------------------------------------
// Database
// -----------------------------------------------------------------------

extern crate tantivy;
// use tantivy;
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::tokenizer::NgramTokenizer;
use tantivy::{Index, ReloadPolicy};

use crate::dir::Dir;
use std::collections::BTreeMap;
use std::path::Path;

pub struct Database {
    schema: Schema,
    index: Index,
}

pub fn init() -> tantivy::Result<Database> {
    let path = Path::new("/usr/local/lib/portal/database/");
    let index_path = MmapDirectory::open(path).unwrap();

    let mut schema_builder = Schema::builder();

    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("ngram3")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();
    let timestamp_options = IntOptions::default()
        .set_stored()
        .set_fast(Cardinality::SingleValue);
    schema_builder.add_text_field("path", text_options);
    // schema_builder.add_u64_field("count", FAST);
    schema_builder.add_u64_field("timestamp", timestamp_options);

    let schema = schema_builder.build();
    let index = Index::open_or_create(index_path, schema.clone())?;

    index
        .tokenizers()
        .register("ngram3", NgramTokenizer::new(3, 3, false));

    Ok(Database { schema, index })
}

pub fn add_entries(database: &Database, entries: Vec<Dir>) {
    println!("Database ADDING {} ENTRIES", entries.len());
    let mut index_writer = database.index.writer(50_000_000).unwrap();

    let path = database.schema.get_field("path").unwrap();
    let timestamp = database.schema.get_field("timestamp").unwrap();

    for e in entries {
        println!("Database ADDING {}", e.path);
        let mut entry = Document::default();
        entry.add_text(path, &e.path);
        entry.add_u64(timestamp, e.timestamp as u64);
        index_writer.add_document(entry);
    }

    index_writer
        .commit()
        .expect("Error adding entries database!");
}

pub fn parse_doc(database: &Database, doc: &Document) -> Dir {
    let mut field_map = BTreeMap::new();

    for (field, field_values) in doc.get_sorted_field_values() {
        let field_name = database.schema.get_field_name(field);
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

pub fn query(database: &Database, query_term: &str) -> Vec<Dir> {
    let path = database.schema.get_field("path").unwrap();
    // let timestamp = database.schema.get_field("timestamp").unwrap();

    let reader = database
        .index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();
    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&database.index, vec![path]);
    let query = query_parser.parse_query(&query_term).unwrap();

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(200))
        .expect("could not parse query");

    // println!("Database FOUND {} RESULTS", top_docs.len());

    let mut results: Vec<Dir> = Vec::new();
    for (_d, doc_address) in top_docs {
        let doc = searcher.doc(doc_address).unwrap();
        let entry = parse_doc(&database, &doc);
        results.push(entry);
    }

    results.sort_by_key(|x| x.timestamp);
    results
}
