use anyhow::{anyhow, Context, Result};

//pub fn total_from_header(header: &HeaderMap) -> Result<usize> {
//    let mut content_range = header
//        .get(header::CONTENT_RANGE)
//        .context("No content-range header found.")?
//        .to_str()?
//        .split("/");
//    let total: usize = content_range.nth(1).context("No total found.")?.parse()?;
//    Ok(total)
//}
//
//pub fn get_query_from_to(page: usize, limit: usize) -> Result<(usize, usize)> {
//    if page == 0 || limit == 0 {
//        return Err(anyhow!("page and limit must greater than 0."));
//    }
//    let from_index = (page - 1) * limit;
//    let to_index = from_index + limit - 1;
//    Ok((from_index, to_index))
//}
//
pub fn total_pages(total: i64, limit: i64) -> i64 {
    if total % limit != 0 {
        (total / limit) + 1
    } else {
        total / limit
    }
}
//
pub fn extract_page_and_limit(page: Option<i64>, limit: Option<i64>) -> (i64, i64) {
    let mut page = page.unwrap_or(1);
    if page <= 0 {
        page = 1;
    }
    let limit = limit.unwrap_or(9999);
    (page, limit)
}

