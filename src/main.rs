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
        .nth(1)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "missing argument"))?;
    let file = std::fs::read_to_string(file)?;
    let format = time::macros::format_description!("[year]-[month]-[day]");
    let date = time::OffsetDateTime::now_utc().format(format)?;
    let req = surf::get(format!("{}{}", URL, "USDGBP=X"));
    let Chart { chart } = surf::client().recv_json(req).await?;
    let usd_price = chart.result[0].meta.regular_market_price;
    for line in file.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let symbol = tokens.next().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "missing symbol")
        })?;
        let beancount_symbol = tokens.next();
        let req = surf::get(format!("{}{}", URL, symbol));
        let Chart { chart } = surf::client().recv_json(req).await?;
        let meta = &chart.result[0].meta;
        let symbol = if let Some(beancount_symbol) = beancount_symbol {
            beancount_symbol
        } else {
            &meta.symbol
        };
        let price_conv = match meta.currency.as_str() {
            "GBp" => 0.01,
            "GBP" => 1.0,
            "USD" => usd_price,
            _ => unreachable!("unknown currency {:?}", chart),
        };
        println!(
            "{} price {} {} GBP",
            date,
            symbol,
            meta.regular_market_price * price_conv,
        );
    }

    Ok(())
}
