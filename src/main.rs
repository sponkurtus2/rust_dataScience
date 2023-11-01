mod create_data_frame;
fn main() {
    // Read a csv file //
    // Load the csv file
    /*let csv_path = "dataFrames/Practice2.csv";

    // Then, we read the file with the CsvReader function (The function comes from polaRS
    let csv_df = CsvReader::from_path(csv_path)
        // We unwrap the content, and prepare it
        .unwrap()
        .finish()
        .unwrap();

    // And finally, we can print it.
    println!("{}", csv_df);*/

    // Prepare a dataFrame from some web
    let _ = create_data_frame::create_df();
    let _ = create_data_frame::show_created_df();
}