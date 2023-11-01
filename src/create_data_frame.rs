use polars::prelude::*;

pub fn create_df() -> Result<DataFrame, PolarsError>{

    let s1 = Series::new("names", &["Mariana", "Carlos", "Shock", "Yoyis"]);
    let s2 = Series::new("values", &[14,19, 4, 19]);
    let df = DataFrame::new(vec![s1, s2])?;

    return Ok(df)


}

pub fn show_created_df() {
    let df = create_df();
    println!("{:?}", df);
}