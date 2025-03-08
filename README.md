<p align="center">
  <img src=https://i.imgur.com/SYBlC49.jpg>
</p>

<h1 align="center">Solana Chess</h1>
<p align="center"><strong>Chess Engine written in Anchor Framework</strong></p>

<div align="center">

  <a href="https://opensource.org/licenses/MIT">![License](https://img.shields.io/badge/License-MIT-yellow.svg)</a>  

</div>

## Features
|         Feature         | Implemented |
|-------------------------|:-----------:|
| Check Legal Moves       |      ✅     |
| Checkmate               |      ✅     |
| Enpassant               |      ✅     |
| Castling                |      ✅     |
| Promotion               |      ✅     |
| Elo                     |      ✅     |
| Time Control            |      ✅     |
| Token Wagers            |      ✅     |
| Elo NFTs                |      ❌     |
| Chess Bot with Clockwork|      ❌     |
| Tournaments             |      ❌     |

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Node.js and npm](https://nodejs.org/)

## Installation

1. Clone the repository:
```sh
git clone https://github.com/yourusername/sol-chess.git
cd sol-chess
```

2. Install dependencies:
```sh
npm install
```

## Local Development and Testing

1. Build the program:
```sh
anchor build
```

2. Run tests:
```sh
anchor test
```

3. Run the client:
```sh
cargo run ./client/
```

## Deploying to X1 Testnet

1. Configure Solana CLI for X1 testnet:
```sh
solana config set -u https://rpc.testnet.x1.xyz
```

2. Generate or import a keypair:
```sh
solana-keygen new --no-passphrase -o ~/.config/solana/id.json
solana config set -k ~/.config/solana/id.json
```

3. Get X1 testnet tokens from the [X1 faucet](https://faucet.testnet.x1.xyz/)

4. Update the Anchor.toml file to use the X1 testnet:
```toml
[provider]
cluster = "https://rpc.testnet.x1.xyz"
wallet = "~/.config/solana/id.json"
```

5. Build the program:
```sh
anchor build
```

6. Deploy the program:
```sh
anchor deploy
```

7. If you encounter the "account data too small" error:
```sh
solana program extend <program id> 15000
```

## Frontend Development Guide

To build a frontend for this chess game, you'll need to interact with the Solana blockchain using the program's IDL (Interface Description Language). The IDL describes all the instructions and accounts that the program uses.

### Getting the IDL

The IDL is located in the `/idl` directory. You can use either:
- `sol_chess.json` - JSON format for JavaScript/TypeScript applications
- `sol_chess.ts` - TypeScript format with type definitions

### Required Dependencies for Frontend

For a web-based frontend, you'll need:

```json
{
  "dependencies": {
    "@solana/web3.js": "^1.73.0",
    "@project-serum/anchor": "^0.26.0",
    "@solana/wallet-adapter-react": "^0.15.28",
    "@solana/wallet-adapter-wallets": "^0.19.10",
    "@solana/wallet-adapter-react-ui": "^0.9.27"
  }
}
```

### Program Instructions

Here are all the instructions available in the program:

| Instruction | Description | Parameters | Accounts |
|-------------|-------------|------------|----------|
| `initialize_user` | Create a new user account | None | payer (signer), user, system_program |
| `initialize_game` | Create a new chess game | game_config: {timer, increment, is_rated, wager} | payer (signer), user, game, system_program, clock |
| `join_game` | Join an existing game | color: White/Black | payer (signer), user, game |
| `move_piece` | Make a chess move | from: {file, rank}, to: {file, rank} | payer (signer), user, adversary_user, game, clock |
| `deposit` | Deposit tokens for wagers | amount: u64 | payer (signer), user, vault, system_program |
| `withdraw` | Withdraw tokens | amount: u64 | payer (signer), user, vault, system_program |
| `leave_game` | Leave a game that hasn't started | None | payer (signer), user, game |
| `resign` | Resign from a game | None | payer (signer), user, adversary_user, game |
| `offer_draw` | Offer or accept a draw | None | payer (signer), user, adversary_user, game |
| `check_time_forfeit` | Claim win when opponent's time runs out | None | payer (signer), user, adversary_user, game, clock |

### Account Structures

#### User Account
```typescript
{
  currentGame: Option<PublicKey>,
  elo: number,
  games: number,
  balance: number
}
```

#### Game Account
```typescript
{
  createdAt: number,
  owner: PublicKey,
  id: number,
  bump: number,
  board: Board,
  gameState: GameState,
  white: Option<PublicKey>,
  black: Option<PublicKey>,
  enpassant: Option<Square>,
  castlingRight: CastlingRight,
  drawState: DrawState,
  gameConfig: GameConfig,
  timeControl: TimeControl
}
```

#### GameConfig
```typescript
{
  timer: number,
  increment: number,
  isRated: boolean,
  wager: Option<number>
}
```

#### TimeControl
```typescript
{
  lastMove: number,
  whiteTimer: number,
  blackTimer: number,
  increment: number
}
```

### Frontend Implementation Steps

1. **Connect to Wallet**:
   ```javascript
   import { useWallet } from '@solana/wallet-adapter-react';
   import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

   // In your component
   const { publicKey, sendTransaction } = useWallet();
   ```

2. **Initialize Program**:
   ```javascript
   import { Program, AnchorProvider } from '@project-serum/anchor';
   import { Connection, PublicKey } from '@solana/web3.js';
   import idl from './idl/sol_chess.json';

   const programId = new PublicKey('9PsU5ntn26Bos8FRtwupQbvoYbchzt8bdQoxLym7AHWB');
   const connection = new Connection('https://rpc.testnet.x1.xyz');
   const provider = new AnchorProvider(connection, wallet, {});
   const program = new Program(idl, programId, provider);
   ```

3. **Create User Account**:
   ```javascript
   const [userPda] = await PublicKey.findProgramAddress(
     [Buffer.from('user'), publicKey.toBuffer()],
     programId
   );

   await program.methods
     .initializeUser()
     .accounts({
       payer: publicKey,
       user: userPda,
       systemProgram: SystemProgram.programId,
     })
     .rpc();
   ```

4. **Create Game**:
   ```javascript
   const [gamePda] = await PublicKey.findProgramAddress(
     [Buffer.from('game'), userPda.toBuffer(), Buffer.from([0, 0, 0, 0, 0, 0, 0, 0])],
     programId
   );

   await program.methods
     .initializeGame({
       timer: 300,
       increment: 5,
       isRated: true,
       wager: new BN(1000),
     })
     .accounts({
       payer: publicKey,
       user: userPda,
       game: gamePda,
       systemProgram: SystemProgram.programId,
       clock: SYSVAR_CLOCK_PUBKEY,
     })
     .rpc();
   ```

5. **Make a Move**:
   ```javascript
   await program.methods
     .movePiece(
       { file: 0, rank: 6 },
       { file: 0, rank: 5 }
     )
     .accounts({
       payer: publicKey,
       user: userPda,
       adversaryUser: adversaryUserPda,
       game: gamePda,
       clock: SYSVAR_CLOCK_PUBKEY,
     })
     .rpc();
   ```

6. **Fetch Game State**:
   ```javascript
   const gameAccount = await program.account.game.fetch(gamePda);
   console.log('Game State:', gameAccount.gameState);
   console.log('Board:', gameAccount.board);
   console.log('Time Control:', gameAccount.timeControl);
   ```

### Chess Board Visualization

For the chess board visualization, you can use:
- [chess.js](https://github.com/jhlywa/chess.js) for chess logic
- [chessboard.js](https://chessboardjs.com/) or [react-chessboard](https://github.com/Clariity/react-chessboard) for the UI

You'll need to convert between the program's board representation and the standard chess notation:

```javascript
// Convert from program's Square to chess.js notation
function squareToNotation(square) {
  const files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
  const ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];
  return files[square.file] + ranks[square.rank];
}

// Convert from chess.js notation to program's Square
function notationToSquare(notation) {
  const files = {'a': 0, 'b': 1, 'c': 2, 'd': 3, 'e': 4, 'f': 5, 'g': 6, 'h': 7};
  const ranks = {'1': 0, '2': 1, '3': 2, '4': 3, '5': 4, '6': 5, '7': 6, '8': 7};
  return {
    file: files[notation[0]],
    rank: ranks[notation[1]]
  };
}
```

## User Guide

### Creating a User Account

Before playing chess, you need to create a user account:

```rust
// Initialize a user account
initialize_user(&client, user)?;
```

### Creating a Game

To create a new chess game:

```rust
// Parameters: user, game, wager amount (optional), timer (seconds), increment (seconds), is_rated
initialize_game(&client, user, game, Some(1000), 300, 5, true)?;
```

The parameters are:
- `user`: Your user account public key
- `game`: The game account public key
- `wager`: Optional token amount to wager (in lamports)
- `timer`: Initial time in seconds (e.g., 300 for 5 minutes)
- `increment`: Time increment in seconds after each move (e.g., 5)
- `is_rated`: Whether the game affects Elo ratings

### Depositing Tokens

To deposit tokens for wagers:

```rust
// Parameters: user, amount (in lamports)
deposit(&client, user, 100000)?;
```

### Joining a Game

To join an existing game:

```rust
// Parameters: user, game, color (White or Black)
join_game(&client, user, game, sol_chess::Color::White)?;
```

### Making Moves

To make a move:

```rust
// Parameters: user, game, from square, to square
let from = sol_chess::Square { file: 0, rank: 6 };
let to = sol_chess::Square { file: 0, rank: 5 };
move_piece(&client, user, game, from, to)?;
```

Chess board coordinates:
- Files (columns): 0-7 (a-h)
- Ranks (rows): 0-7 (1-8)
- Example: e4 would be { file: 4, rank: 3 }

### Time Control

The game includes time control with the following features:
- Initial timer for each player (set during game creation)
- Time increment after each move
- Automatic loss if time runs out

To claim a win when your opponent's time runs out:

```rust
// Parameters: user, adversary_user, game
check_time_forfeit(&client, user, adversary_user, game)?;
```

### Resigning

To resign from a game:

```rust
// Parameters: user, adversary_user, game
resign(&client, user, adversary_user, game)?;
```

### Offering a Draw

To offer or accept a draw:

```rust
// Parameters: user, adversary_user, game
offer_draw(&client, user, adversary_user, game)?;
```

### Leaving a Game

To leave a game that hasn't started yet:

```rust
// Parameters: user, game
leave_game(&client, user, game)?;
```

### Withdrawing Tokens

To withdraw tokens from your account:

```rust
// Parameters: user, amount (in lamports)
withdraw(&client, user, 50000)?;
```

## Note 

Anchor will generate a broken idl, use the one provided in `/idl`

## License

This project is licensed under the MIT License - see the LICENSE file for details.
