# Clockwork

Clockwork is a [Alloy](https://github.com/alloy-rs/alloy)-based sniper bot, written in Rust, that interacts with the [time.fun](https://time.fun) smart contract on the Base network. It monitors for new time.fun users and snipes shares.

I am open sourcing because time.fun sniping is kinda boring and I haven't seen many examples of Alloy used in the wild. I have used this bot in production to snipe (for a couple hours?) for approx a $300 profit. Have fun.

## Installation

Ensure you have Rust nightly installed on your system.

Then, clone the repository and build the project:

```sh
git clone https://github.com/mteam88/clockwork.git
cd clockwork
cargo build --release
```

## Configuration

1. Copy the `.env_example` file to `.env`:

```sh
cp .env_example .env
```

2. Edit the `.env` file and fill in the necessary information:

```
PRIVATE_KEY=your_ethereum_private_key
RPC_URL="wss://base-rpc.publicnode.com"
NUM_MINUTES=20
TIMEFUN_ADDRESS="0x428aeF7fB31E4E86162D62d4530a4dd7232D953D"
```

- Replace `your_ethereum_private_key` with your actual Ethereum private key. Do not include the 0x prefix. This private key should be adequately funded with ETH to cover the gas fees and share purchases.
- Adjust `NUM_MINUTES` if you want to change the number of minutes (shares) to buy.
- You can modify the `RPC_URL` if you want to use a different node provider.

**Note:** Never commit your `.env` file or share your private key.

## Usage

To run the bot, use the following command:

```sh
cargo run --release
```

The bot will start monitoring for SharesCreated events. When an event is detected and the creator has a minimum balance of 0.0005 ETH (this is to block spammers hehe), it will automatically purchase the specified number of shares.

**Note:** You will need to sell these shares yourselves either from the time.fun website or from the TimeBasedExperience contract directly.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. I will not commit any more to this project myself.

## Disclaimer

This software interacts with blockchain technology and involves financial transactions. Use at your own risk. Always verify the code and understand the implications before running it, especially when dealing with real funds.