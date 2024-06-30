use anyhow::{anyhow, Context, Result};
use reqwest::header::{self, HeaderMap};

pub fn range_and_total_from_header(header: &HeaderMap) -> Result<(String, u32)> {
    let mut content_range = header
        .get(header::CONTENT_RANGE)
        .context("No content-range header found.")?
        .to_str()?
        .split("/");
    let range = content_range.next().context("No range found.")?.to_string();
    let total: u32 = content_range.next().context("No total found.")?.parse()?;
    Ok((range, total))
}

pub fn get_query_from_to(page: usize, limit: usize) -> Result<(usize, usize)> {
    if page == 0 || limit == 0 {
        return Err(anyhow!("page and limit must greater than 0."));
    }
    let from_index = (page - 1) * limit;
    let to_index = from_index + limit - 1;
    Ok((from_index, to_index))
}
