mod programs;
use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram, UpdateArgs};

#[cfg(test)]
mod tests {
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message, signature::{read_keypair_file, Keypair, Signer}, system_program, transaction::Transaction
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram};

    const RPC_URL: &str = "https://api.devnet.solana.com";
    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin-wallet.json").expect("Couldn't find wallet file");       
         let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        let args = CompleteArgs {
            github: b"Asharma171".to_vec(),
        };
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Checkout your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );

        // https://explorer.solana.com/tx/617UxdnT6UrkBTVtnhz4Unp5fccipknainnJnqup7mzeyQGgDPgfJb2oUUUw7D5tyzFtU2uwt6q92jQLvrbSqmpe?cluster=devnet
    }
//-----------------------------last point --------------------------------------

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
        // 7McN8iLufoPX6VWMCARbGzUKNrxGuYCxUBu3yE5QjJHA
    }
    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here: ");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
        // https://explorer.solana.com/tx/54KTDX8rGDhrX2Dwi9SS8dMTC5Fk2RwwQhXfgGDMD9F7oxysXDvWbjKbS3bFHYXT28neKrJVuRTUu86457T61JM6?cluster=devnet
    }

    #[test]
    fn transfer_sol() {
        let keypair: Keypair =
            read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("8TYzRia4pJ349AhAAVJcxzBwS7CdRMEVfbhaH3X4A6Z9").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get ");
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
        // 0.1 sol Transfer https://explorer.solana.com/tx/3w463fgfqTvTYQLJNRwyQMnC7h4pYWyehSoeVTL6Z9QRKQjmzCNg1v1v3vRhojiukFbCmbmQJmJTB6DdHVqxdx6U?cluster=devnet
        // Complete https://explorer.solana.com/tx/3FNx1miBF8kzvjKZ5jCVTtGQ42ofacAvccvWuj92ftqexPLJt7jrCa4DbZM3avSfY2WiXro9gTiNn7jTUCd9EYM?cluster=devnet
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }


    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}
