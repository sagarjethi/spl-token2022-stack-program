# Solana SPL Staker Program with Token 2022 Support

A modern Solana staking program built with Anchor that supports both **SPL Token** and **Token 2022** standards, enabling users to stake tokens and earn rewards with advanced token features.

## 🚀 Features

- **Dual Token Support**: Works with both SPL Token and Token 2022 programs
- **Backward Compatible**: Existing SPL Token functionality remains unchanged
- **Token 2022 Extensions**: Supports advanced features like transfer fees, confidential transfers, and more
- **Secure Staking**: Users can stake tokens and receive reward tokens
- **Flexible Architecture**: Easy to extend for future token program versions

## 📋 Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.14.17+)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) (v0.31.1+)
- [Node.js](https://nodejs.org/) (v16+)
- [Yarn](https://yarnpkg.com/) or npm

## 🛠️ Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/sagarjethi/spl-token2022-stack-program.git
   cd spl-token2022-stack-program
   ```

2. **Install dependencies**
   ```bash
   yarn install
   # or
   npm install
   ```

3. **Build the program**
   ```bash
   anchor build
   ```

4. **Run tests**
   ```bash
   anchor test
   ```

## 🏗️ Program Architecture

### Core Functions

The staker program provides three main functions:

1. **`create_myspl_ata`** - Creates an Associated Token Account for the program to hold user tokens
2. **`stake`** - Transfers user tokens to the program and mints reward tokens
3. **`unstake`** - Burns reward tokens and returns the original staked tokens

### Token Program Support

| Token Program | Program ID | Features |
|---------------|------------|----------|
| SPL Token | `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` | Standard token functionality |
| Token 2022 | `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | Advanced extensions, transfer fees, confidential transfers |

## 💻 Usage

### Basic Staking Flow

```typescript
import { TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID } from '@solana/spl-token';

// For SPL Token
const tokenProgram = TOKEN_PROGRAM_ID;

// For Token 2022
const tokenProgram = TOKEN_2022_PROGRAM_ID;

// Stake tokens
await program.rpc.stake(
  stakePDABump,
  beefBagBump,
  new anchor.BN(amount),
  {
    accounts: {
      tokenProgram: tokenProgram,
      // ... other accounts
    }
  }
);
```

### Program Accounts

The program uses the following key accounts:

- **Reward Mint**: The token minted as rewards for staking
- **Stake Token**: The token users stake to earn rewards
- **Program ATA**: Associated Token Account where staked tokens are held
- **User ATA**: User's Associated Token Account for receiving rewards

## 🧪 Testing

The project includes comprehensive tests for both token programs:

```bash
# Run all tests
anchor test

# Run specific test file
npx ts-mocha tests/staker_token2022.ts
```

### Test Coverage

- ✅ Account creation with both token programs
- ✅ Staking functionality with SPL Token and Token 2022
- ✅ Unstaking functionality with both token programs
- ✅ Error handling and edge cases

## 📁 Project Structure

```
├── programs/
│   └── staker/
│       ├── src/
│       │   └── lib.rs          # Main program logic
│       └── Cargo.toml          # Rust dependencies
├── tests/
│   ├── staker.ts               # Original tests
│   ├── staker_token2022.ts     # Token 2022 tests
│   ├── token_helper.ts         # Token utilities
│   └── user.ts                 # User management
├── scripts/
│   ├── programHelper/          # Program utilities
│   └── spl/                    # SPL token scripts
├── migrations/
│   └── deploy.ts               # Deployment script
└── TOKEN_2022_MIGRATION.md     # Migration documentation
```

## 🔧 Configuration

### Environment Setup

1. Copy `.env.example` to `.env`
2. Configure your Solana cluster and wallet
3. Set up token metadata in `scripts/spl/splHelper/consts.ts`

### Anchor Configuration

The program is configured in `Anchor.toml`:

```toml
[programs.devnet]
staker = "2vteEBtJopYp8Shc1kbD29WCjAZezp8mJMoDkmX5xGca"

[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"
```

## 🚀 Deployment

1. **Deploy to devnet**
   ```bash
   anchor deploy --provider.cluster devnet
   ```

2. **Deploy to mainnet**
   ```bash
   anchor deploy --provider.cluster mainnet-beta
   ```

## 🔒 Security

- **Audited**: The program uses well-tested Anchor and SPL token libraries
- **Secure CPI**: All token operations use secure Cross-Program Invocations
- **Account Validation**: Proper account ownership and type validation
- **PDA Security**: Program Derived Addresses for secure account management

## 📚 Documentation

- [Token 2022 Migration Guide](TOKEN_2022_MIGRATION.md) - Detailed migration documentation
- [Solana Token Program](https://docs.solana.com/developing/programming-model/overview) - Solana programming model
- [Anchor Framework](https://www.anchor-lang.com/) - Anchor documentation
- [SPL Token](https://spl.solana.com/token) - SPL Token documentation

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Solana Labs](https://solana.com/) for the Solana blockchain
- [Anchor Framework](https://www.anchor-lang.com/) for the development framework
- [SPL Token Program](https://spl.solana.com/token) for token standards

## 📞 Support

If you have any questions or need help:

- Open an [issue](https://github.com/sagarjethi/spl-token2022-stack-program/issues)
- Check the [documentation](TOKEN_2022_MIGRATION.md)
- Review the [tests](tests/) for usage examples

---

**Built with ❤️ for the Solana ecosystem**