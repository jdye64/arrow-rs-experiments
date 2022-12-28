use arrow_csv::ReaderBuilder;
use std::fs::File;

fn main() {

    let file = File::open("/home/jeremy/Development/arrow-experiments/tips.csv").unwrap();

    // create a builder, inferring the schema with the first 1 records
    let builder = ReaderBuilder::new()
        .has_header(true)
        .with_delimiter(b',')
        .with_batch_size(10)
        .infer_schema(Some(1));

    let mut reader = builder.build(file).unwrap();

    println!("Schema: {:?}", reader);

    let batch = reader.next().unwrap().unwrap();
    println!("Batch: {:?}", batch)
}
