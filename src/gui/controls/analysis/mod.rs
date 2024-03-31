pub mod cdifile_overview;


// anaylsis datatable ideas
// - show all sectors in a table
// - with a filter for sector type
// - with a filter for video sector type
// - with a filter for audio sector properties

// what is the best sturcture for this to allow reuse of the table and filters?

// Table {
//   columns: Vec<Column>,
//   source: Vec<T>,
// }

// Column {
//   name: String,
//   type: ColumnType,
//   value: Option<T>,
//}
// Column types
// - Text
// - Numeric
// - Date
// - Checkbox
// - Icon
