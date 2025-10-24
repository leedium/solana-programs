use litesvm::LiteSVM;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signer::keypair::{Keypair, Signer};
#[test]
fn test_hello_world() {
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();

    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL);

    let balance = svm.get_balance(&payer.pubkey()).unwrap();

    println!("The balance is: {}", &balance);

    assert!(balance == LAMPORTS_PER_SOL, "Balance airdropped")
}
