# Pokémon TCG IO API Wrapper
This is a wrapper for the REST API of [pokemontcg.io](https://pokemontcg.io/) that uses builder patterns inspired by [reqwest](https://github.com/seanmonstar/reqwest).

# Usage
## Configuration
TODO

## Using With an API Key
```rust
let client = Client::with_api_key("API_KEY");
```

## Cards
### Fetching a single card
```rust
let client = Client::with_api_key("API_KEY");
let card = client.get_card("swsh4-183").await?;
match card {
	Some(c) => println!("{:?}", c),
	None => println!("No card found!")
}
```
### Searching for cards
```rust
let client = Client::with_api_key("API_KEY");
// Fetch cards with no filters
let cards = client.search_cards().send().await?;
match cards {
	Some(c) => println!("{:?}", c),
	None => println!("No cards found!")
}

// search_cards returns a builder with a number of methods to add filters
let cards = client
	.search_cards()
	.query("name:charizard")
	.page(2)
	.page_size(5)
	.order_by("rarity")
	.select("hp,flavor_text")
	.send()
	.await?;
match cards {
	Some(c) => println!("{:?}", c),
	None => println!("No cards found!")
}
```

## Sets
All the same functions for cards exist for sets as well
```rust
let client = Client::with_api_key("API_KEY");
let set = client.get_set("swsh4").send().await?;

match set {
	Some(c) => println!("{:?}", c),
	None => println!("No set found!")
}

// Fetch sets with no filters
let sets = client.search_sets().send().await?;
match sets {
	Some(c) => println!("{:?}", c),
	None => println!("No sets found!")
}

// search_sets returns a builder with a number of methods to add filters
let sets = client
	.search_sets()
	.query("series:Sword & Shield")
	.page(4)
	.page_size(1)
	.order_by("total")
	.select("printed_total,total")
	.send()
	.await?;
match sets {
	Some(c) => println!("{:?}", c),
	None => println!("No sets found!")
}
```
## Types, Subtypes, Supertypes and Rarities
These types only provide methods for fetching them all
```rust
let client = Client::with_api_key("API_KEY");

// All are Vec<String>
let types = client.get_types().send().await?;
let subtypes = client.get_subtypes().send().await?;
let supertypes = client.get_supertypes().send().await?;
let rarities = client.get_rarities().send().await?;
```