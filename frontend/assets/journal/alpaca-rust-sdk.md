# Alpaca API Rust SDK

I want to design an SDK that would wrap the [Alpaca API](https://docs.alpaca.markets/reference/stockauctions-1) for the Rust ecosystem. Initially this was a necessity for my trading algorithms, but turned into an open source project for the community.

## HTTP Requests

They use a Restful API architecture, which is great because we can use any practically any programming language. We have a decision to make here. Do we use an async http client OR a synchronous one? Software engineering is the art of trade offs, and we have to weigh the pros and cons of each approach. I have decided to use the synchronous one.

## Design

In frontend design we would break the project in to three parts:

- User Interface
- User Experience
- Developer Experience

There is a similar design hierarchy when making a library:

- User Experience
- Adaptability
- Developer Experience

We have all experienced a great library and a poor one. The difference is usually felt in the user experience, but sometimes the use case is just too niche for the library to adapt to. This results in hacky workarounds. I include developer experience in the design hierarchy because there has to be some sort of attention on the development side of building things. The people who have to improve and maintain the project should have a stake in the decision making. Simply saying "yes" to each new feature, no matter the overhead, is not sustainable. And there has to be room for code base improvements in the future, so we can't just keep building to serve the first two parts.

## Implementation

For this library, I chose to use a variation of the "Builder pattern", [Wiki](https://en.wikipedia.org/wiki/Builder_pattern), which is not too foreign in Rust.

Most of the endpoints provided by Alpaca are customized through query strings. Here is an example:

```text
"https://data.alpaca.markets/v2/stocks/bars/latest?symbols=AAPL%2CTSLA&feed=iex&currency=USD"
```

Here is the our library code for that:

```rust
let query = LatestTradesQuery::new(vec!["AAPL", "TSLA"])
    .feed("iex")
    .currency("USD")
    .send()
    .unwrap();
```

This would return a serde deserialized `LatestTrades` struct that represents what comes back from Alpaca. The arguments are what is considered **required** by the api, and the method chains are the customizations.

## Serde

Serde is a nifty library in Rust. This can help us turn some of these mutable structs into JSON objects that can be sent to Alpaca. Here is an example when building the order creation request:

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderQuery<'a> {
    pub symbol: &'a str,
    pub side: String,
    pub r#type: String,
    pub time_in_force: String,
    pub extend_hours: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_percent: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<TakeProfit<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<StopLoss<'a>>,
}
```

This is fantastic. We have the ability to omit fields if they are not needed. Serde also offers a features for customizing how to serialize and deserialize objects, though we won't need those in this case. Now we can send this to Alpaca.

```rust
pub fn send(self, account_type: AccountType) -> Result<Order, ureq::Error> {
    let url = match account_type {
        AccountType::Live => "https://api.alpaca.markets/v2/orders",
        AccountType::Paper => "https://paper-api.alpaca.markets/v2/orders",
    };

    let response = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(&self)?;

    let order = response.into_json()?;
    Ok(order)
}
```

## Conclusion

Our builder pattern is working out great for this library as it allows the user to be as flexible as possible, and allows us to easily deal with their customizations. This should also feel like a natural extension of the Rust ecosystem. So that's all 3 categories, user interface, adaptability, and developer experience.

The library is available at [Crates.io](https://crates.io/crates/alpaca_api_client). It has gone through a couple design changes as I learn more about Rust and programming principles, but this is my favorite so far.
