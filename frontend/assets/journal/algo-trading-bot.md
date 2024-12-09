# Algo Trading Bot

The stock market is a data making machine. Every day, there are millions of trades happening, and even more is happening around those trades. This has always fascinated me. I wanted to get a better understanding of what's going on in the market, so I built a bot that would analyze the market and make decisions based on the data. There are many different ways to do this, and I have tried dozens. This is just a brief overview of what I did. Any algorithm can be plugged into the pipeline.

I already created an Alpaca API SDK in Rust here:

<https://crates.io/crates/alpaca_api_client>

I have also created a tiny but helpful technical indicators library here:

<https://crates.io/crates/tindi>

## What is a Bot?

In this case, a bot is the all encompassing machine that reads market **data**, analyzes it with an **algorithm**, **decides** to buy or sell a stock, and then **executes** the order.

## Data

Of course, we have to start with the data. This moves the world and is the heartbeat of the trading bot.

### Time Frames

Most stock market algos are targeted at a time frame. A time frame is a period of data at a certain interval of time. For examples, the 5 minute time frame is a 5 minute interval, and each period is the price of the stock at the end of the 5 minutes. Combine this with a "candle", you can get the high, low, open, and close of the stock in that one period at that time frame.

### Technical Indicators

You can chart the stock market similar to an algebra teacher charts a graph. Give that the market is just price points over time, this provides a ton of room for mathematical analysis. These are typically called technical indicators. A few examples are Bollinger Bands, RSI, and MACD. I tend to stay away from such things, as they are just visuals and don't really provide any real information, but there are many ways to use them and some people do so with success, so find a way that works for you.

### Reading the Data

So what good is all this data if we can't do anything with it? This is where the bot comes in. The bot will read the data and make a decision based on the data. This is where you plug in your own logic. Into the machine goes the stock market data at a desired time frame and out comes out with a **Signal**.

### Example

```rust
pub const WATCHLIST: [&str; 26] = [
    "XLE", "XLF", "XLY", "XLV", "XLP", "XLB", "XLI", "XLC", "XLK", "XLU", "XLRE", "XTN", "XME",
    "SMH", "VWO", "VIS", "VYM", "VO", "VB", "VPU", "VFH", "VAW", "VUG", "VDE", "VDC", "VOO",
];

// * Gather data
let market = StockMarketBuilder::new(WATCHLIST.to_vec(), TimeFrame::OneMinute)
    .build()
    .context("Failed to build market data")?
    .market_hours_only();
```

## Algorithm

At the heart of the bot is the particular algorithm that is deciding on which side (long, short, none), to take on a stock.

### Input

The algorithm is fed the raw data from the market, but scoped to a time frame and watch list.

### Logic

In the middles is where the custom logic is. For a quick example, let's take a look at an RSI algorithm. This would read the input data for say the past 40 days of AAPL and calculate the RSI for that period. If the RSI is above 70, then we buy. If it is below 30, then we sell. If it is between 30 and 70, then we do nothing.

For this to work, that means we would first need to create a function that computes the RSI for 40 periods (likely a custom amount of periods though). Here is a snippet from the function in my Tindi library:

```rust
pub fn relative_strength_index(data: &[f32]) -> f32 {
    // Calculate gains and losses
    let mut gains = Vec::new();
    let mut losses = Vec::new();
    for i in 0..data.len() - 1 {
        let diff = &data[i + 1] - &data[i];
        if diff > 0.0 {
            gains.push(diff);
            losses.push(0.0);
        } else {
            losses.push(diff.abs());
            gains.push(0.0);
        }
    }

    // Calculate rsi
    let avg_gain = simple_moving_average(&gains);
    let avg_loss = simple_moving_average(&losses);

    if avg_loss == 0.0 {
        return 100.0 - (100.0 / (1.0 + avg_gain));
    }

    100.0 - (100.0 / (1.0 + (avg_gain / avg_loss)))
}
```

As you can see, you also need a simple moving average function as well. Next, the algorithm needs the literal if block that says "is this over 70 or below 30?".

```rust
if rsi > 70.0 {
    return Signal::Buy;
} else if rsi < 30.0 {
    return Signal::Sell;
} else {
    return Signal::None;
}
```

### Output

The output of the algorithm is a signal. This signal is then passed to the next leg of the trading bot, which is the part concerned with placing the order.

## Execution

The execution part of the bot is where the order is placed. This would seem as simple as placing a buy or sell order with the broker, but that is not how algo trading works. Each order has to have an exit plan, otherwise we are just buying and holding forever unless we MANUALLY sell the asset. What is the point of that?

### Entry

Orders come in different flavors. For the purpose of this, we will look at a simple bracket order. I created a function that takes the signals and some configs and executes the order. What this is saying is "I want 10 shares, and when the price moves up 2x, I want to sell for profit. When the price moves down 1x, I want to sell for loss."

```rust
let order_forms = build_bracket_order_forms(
    &signals,
    BracketOrderBuilderConfig {
        qty: "10.0".to_string(),
        profit_multiplier: 2.0,
        loss_multiplier: 1.0,
    },
);
```

### Exit

Luckily, we do not have to concern ourselves with the selling part in the code. The brokerage system will manage the order for us.

## Running the Bot

So that is all great, but how do we run the bot? This actually turns out to be more involved than the algo part. Here are the parts that maintain the bots health and functionality.

### Live Feed

We could run the above just once, but that would defeat the purpose of an autonomous trading bot. We need it to react to the live market data all day. Alpaca provides us a websocket to connect to. Here is a snippet of that code. We need to account for connection errors and reconnection.

```rust
loop {
    match run_websocket_stream(&self.secrets).await {
        Ok(_) => {
            // Connection closed normally
            println!("WebSocket connection closed, attempting to reconnect...");
            sentry::capture_message(
                "WebSocket connection closed, attempting to reconnect...",
                sentry::Level::Warning,
            );
            retry_count = 0;
        }
        Err(e) => {
            println!("WebSocket error: {}", e);
            capture_anyhow(&e);

            retry_count += 1;
            if retry_count >= max_retries {
                println!("Max retry attempts reached. Stopping.");
                sentry::capture_message(
                    "Max retry attempts reached. Stopping.",
                    sentry::Level::Error,
                );
                break;
            }
        }
    }

    // Exponential backoff: 1s, 2s, 4s, 8s, etc.
    let backoff_duration = Duration::from_secs(2u64.pow(retry_count as u32));
    println!("Waiting {:?} before reconnecting...", backoff_duration);
    sleep(backoff_duration).await;
}
```

```rust
// * Connect to Alpaca WebSocket API
let (ws_stream, _) = connect_async("wss://stream.data.alpaca.markets/v2/iex")
    .await
    .context("Failed to connect to Alpaca WebSocket API")?;
let (mut write, mut read) = ws_stream.split();

// * Send authentication
let key = secrets
    .get("APCA_API_KEY_ID")
    .expect("Missing APCA_API_KEY_ID");
let secret = secrets
    .get("APCA_API_SECRET_KEY")
    .expect("Missing APCA_API_SECRET_KEY");
let auth_msg = json!({
    "action": "auth",
    "key": key,
    "secret":secret
});
write
    .send(Message::Text(auth_msg.to_string()))
    .await
    .context("Failed to send authentication message")?;

// * Subscribe to a minute bar
let subscribe_msg = json!({
    "action": "subscribe",
    "bars": [
        "XLE", "XLF", "XLY", "XLV", "XLP", "XLB", "XLI", "XLC", "XLK", "XLU", "XLRE", "XTN", "XME",
        "SMH", "VWO", "VIS", "VYM", "VO", "VB", "VPU", "VFH", "VAW", "VUG", "VDE", "VDC", "VOO"
    ]
});
write
    .send(Message::Text(subscribe_msg.to_string()))
    .await
    .context("Failed to send subscribe message")?;

// * Handle incoming messages
let mut previous_candles: Vec<(String, Candle)> = Vec::new();
while let Some(msg) = read.next().await {
    if msg.is_err() {
        println!("Failed to read message from Alpaca WebSocket API");
        return Err(anyhow::anyhow!(
            "Failed to read message from Alpaca WebSocket API"
        ));
    }
    .../
```

### Analysis

As with any program, we will want to see what is happening in real time. This is where a Database and Error reporting system become crucial for keeping track of what is happening. We do not want this bot to get out of hand. There are already safeguards in the code itself, but we want to be able to track everything as well.

I use [Turso DB](https://turso.tech/) for my database entries. Things I track are the following:

- Order time frame
- Order Side
- Timestamp of order
- Why the order was placed
- Version of bot
- Current price of stock

...and some other relevant fields. I do this for anything in the system that I consider important.

### Error Reporting

An awesome program that works with Rust and leverages the Sentry SDK is [GlitchTip](https://glitchtip.com/). You can also plug in the **anyhow** package to simplify error reporting. So any time the bot would throw an error or panic, we can see what is happening right in the dashboard.

## Hosting

Last question is, where are we gonna host this thing? We could run it on our own machine, but that would mean our computer and process would always need to be running. Luckily, there are several options out there. I chose to use [Shuttle](https://www.shuttle.dev/). It is a framework for building and deploying all things Rust. It is very simple to use, and has a very good community. I have been using it for a while now, and it is a great choice. This website you are on is likely running on shuttle right now.

## Conclusion

Making a trading algo bot is a huge challenge, but it is also a great learning experience. I hope this is helpful to anyone that is interested in trading algorithms. My advice is to get all the things around the algorithm working and stable FIRST. Data feed integrity, error handling and reporting, live feed maintenance, and hosting. Everything else will be experimenting with the stock market and finding what algo works for you.
