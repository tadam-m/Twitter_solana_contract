### Solana lifecycle :

# Start the local ledger.
solana-test-validator

# Then, on a separate terminal session.
anchor build
anchor deploy
anchor run test

### Anchor test cmd

# Start the local ledger.
solana-test-validator

# Then, on a separate terminal session.
anchor build
anchor deploy
anchor run test

### Anchor localnet cmd
solana-test-validator --reset
anchor build
anchor deploy

# The local ledger will stay active after deployment.


### AccountInfo:
This is a low-level Solana structure that can represent any account. When using AccountInfo, the account's data will be an unparsed array of bytes.
### Account: 
This is an account type provided by Anchor. It wraps the AccountInfo in another struct that parses the data according to an account struct provided as a generic type. In the example above, Account<'info, Tweet> means this is an account of type Tweet and the data should be parsed accordingly.
### Signer: 
This is the same as the AccountInfo type except we're also saying this account should sign the instruction.
