// # Basic Example
//
// This example covers the basic functionalities of
// tantivy.
//
// We will :
// - define our schema
// = create an index in a directory
// - index few documents in our index
// - search for the best document matchings "sea whale"
// - retrieve the best document original content.

// -----------------------------------------------------------------------
// Tantivy
// -----------------------------------------------------------------------

// #[macro_use]
use tantivy;
use tantivy::collector::Count;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;
use tempdir::TempDir;

pub fn init() -> tantivy::Result<(Schema, TempDir)> {
    let index_path = TempDir::new("portal")?;

    let mut schema_builder = Schema::builder();

    // let count_options = IntOptions::default()
    //     .set_stored()
    //     .set_fast(Cardinality::SingleValue);
    // let timestamp_options = IntOptions::default()
    //     .set_stored()
    //     .set_fast(Cardinality::SingleValue);
    schema_builder.add_text_field("directory", TEXT | STORED);
    // schema_builder.add_u64_field("count", FAST);
    schema_builder.add_u64_field("timestamp", FAST);

    let schema = schema_builder.build();

    Ok((schema, index_path))
}

pub fn seed(schema: &Schema, index_path: &TempDir) -> tantivy::Result<Index> {
    let index = Index::create_in_dir(&index_path, schema.clone()).unwrap();

    let mut index_writer = index.writer(50_000_000).unwrap();

    let directory = schema.get_field("directory").unwrap();
    let timestamp = schema.get_field("timestamp").unwrap();

    let mut entry_1 = Document::default();
    entry_1.add_text(directory, "/Users/dmix/dev/_rust/portal");
    entry_1.add_u64(timestamp, 1557849352);
    index_writer.add_document(entry_1);

    let mut entry_2 = Document::default();
    entry_2.add_text(directory, "/Users/dmix/dev/_elixir/issues ");
    entry_2.add_u64(timestamp, 1561657040);
    index_writer.add_document(entry_2);

    let mut entry_3 = Document::default();
    entry_3.add_text(directory, "/Users/dmix/dev/_nim/karax/examples");
    entry_3.add_u64(timestamp, 1549258325);
    index_writer.add_document(entry_3);

    index_writer.commit().is_ok();
    // let reader = index.reader().unwrap();
    // let searcher = reader.searcher();
    // let reader = searcher.segment_reader(0);
    // println!("xxx {:?}", reader);
    // assert_eq!(reader.num_docs(), 4);

    // let reader = index
    //     .reader_builder()
    //     .reload_policy(ReloadPolicy::OnCommit)
    //     .try_into()
    //     .unwrap();
    let reader = index.reader().unwrap();
    let searcher = reader.searcher();

    // let directory = schema.get_field("directory").unwrap();
    // let timestamp = schema.get_field("timestamp").unwrap();

    let query_parser = QueryParser::for_index(&index, vec![directory]);

    let query = query_parser.parse_query("Users").unwrap();

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(2))
        .expect("could not parse query");

    println!("TopDocs results: {:?}", top_docs);
    for (_d, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        println!("{}", schema.to_json(&retrieved_doc));
    }
    Ok(index)
}

pub fn read(schema: &Schema, index: &Index) {}
