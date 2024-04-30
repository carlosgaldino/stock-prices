use surf;
const URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart/";

#[derive(Debug, serde::Deserialize)]
struct Chart {
    chart: ChartResult,
}

#[derive(Debug, serde::Deserialize)]
struct ChartResult {
    result: Vec<Meta>,
}

#[derive(Debug, serde::Deserialize)]
struct Meta {
    meta: Metadata,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    symbol: String,
    currency: String,
    regular_market_price: f32,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "missing argument"))?;
    let file = std::fs::read_to_string(file)?;
    for line in file.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let symbol = tokens.next().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "missing symbol")
        })?;
        let beancount_symbol = tokens.next();
        let req = surf::get(format!("{}{}", URL, symbol));
        let Chart { chart } = surf::client().recv_json(req).await?;
        dbg!(&chart);
    }

    Ok(())
}
