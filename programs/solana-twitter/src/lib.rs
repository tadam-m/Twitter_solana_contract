use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("53uHc3xziZrgYCUnzagfqqRi9rJY4yYG6TevrAwqrcfN");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> ProgramResult {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet; //access the tweet account via ctx.accounts.tweet
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap(); //get the solana clock 
        
        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into())
        }
    
        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into())
        }

        tweet.author = *author.key; // get the autor public key
        tweet.timestamp = clock.unix_timestamp; // get the timestamp of the tweet message 
        tweet.topic = topic;
        tweet.content = content;

        Ok(())
    }

}

#[error]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}

#[derive(Accounts)]
pub struct SendTweet<'info> { // its public key should be provided when sending the instruction.
    #[account(init, payer = author, space = Tweet::LEN)] // define the payer and the space the author need to pay for the rent-exempt money
    pub tweet: Account<'info, Tweet>, // This is the account that the instruction will create. -> "here's a public key that I own, create a Tweet account there for me please".
    #[account(mut)] //constraint on the system_program to ensure it really is the official System Program from Solana.
    pub author: Signer<'info>, //we need to know who is sending the tweet and so we need the signature 
    #[account(address = system_program::ID)]   
    pub system_program: AccountInfo<'info>, //official System Program from Solana, initialize the Tweet account and figure out how much money we need to be rent-exempt.
}

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

const DISCRIMINATOR_LENGTH: usize = 8; //discriminator stores the type of the account and so 8 bytes
const PUBLIC_KEY_LENGTH: usize = 32; // public key need to store 32 bytes
const TIMESTAMP_LENGTH: usize = 8;  // timestamp need to store 8 bytes
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.


// 3. Add a constant on the Tweet account that provides its total size.
impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content.
}