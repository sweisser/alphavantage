use alphavantage::time_series::IntradayInterval;
use alphavantage::Client;
use std::env;
use structopt::StructOpt;

const TOKEN_ENV_KEY: &str = "ALPHAVANTAGE_TOKEN";

#[derive(StructOpt)]
struct Cli {
    #[structopt(
        short = "t",
        long = "token",
        help = "API token (can use the ALPHAVANTAGE_TOKEN env var instead)"
    )]
    token: Option<String>,
    #[structopt(help = "period (1min, 5min, 15min, 30min, hourly, daily, weekly or monthly)")]
    period: String,
    #[structopt(help = "stock symbol (e.g. AAPL)")]
    symbol: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let token = args
        .token
        .or_else(|| env::var(TOKEN_ENV_KEY).ok())
        .ok_or_else(|| "missing token")?;

    let symbol = &args.symbol;
    let client = Client::new(&token);

    let full = false;
    let time_series = match args.period.as_str() {
        "1min" => {
            client
                .get_time_series_intraday(symbol, IntradayInterval::OneMinute, full)
                .await
        }
        "5min" => {
            client
                .get_time_series_intraday(symbol, IntradayInterval::FiveMinutes, full)
                .await
        }
        "15min" => {
            client
                .get_time_series_intraday(symbol, IntradayInterval::FifteenMinutes, full)
                .await
        }
        "30min" => {
            client
                .get_time_series_intraday(symbol, IntradayInterval::ThirtyMinutes, full)
                .await
        }
        "hourly" => {
            client
                .get_time_series_intraday(symbol, IntradayInterval::SixtyMinutes, full)
                .await
        }
        "daily" => client.get_time_series_daily(symbol, full).await,
        "weekly" => client.get_time_series_weekly(symbol, full).await,
        "monthly" => client.get_time_series_monthly(symbol, full).await,
        _ => Err(format!("unknown period {}", args.period))?,
    }?;

    println!(
        "{} (updated: {})\n",
        time_series.symbol, time_series.last_refreshed
    );
    for entry in time_series.entries {
        println!("{}:", entry.date);
        println!("  open: {}", entry.open);
        println!("  high: {}", entry.high);
        println!("  low: {}", entry.low);
        println!("  close: {}", entry.close);
        println!("  volume: {}", entry.volume);
    }
    Ok(())
}
