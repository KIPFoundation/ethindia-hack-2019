extern crate meval;
extern crate web3;
use web3::futures::Future;
use std::io::{stdin,stdout,Write};
use web3::contract::{Contract, Options};
use web3::types::{Address, H160, H256};


fn main() {
    // logic for mathematic operations
    let sum = "(-3) + 5.6";
    let sub = "5.9 - 5.3";
    let multi = "10.4 *6.3";
    let div = "6 / 5";
    let rem = "6 % 6";
    let sqt = "sqrt(16)";
    let pow = "exp(3)";
    let nlog = "ln(1)";
    let absolute = "abs(-8)";
    let round = "round(4.8584)";
    let e = "e";
    let pi = "pi";
    let sign = "signum (-0.1)";

    //passing result as a string
    let res = meval::eval_str(sum).unwrap();
    println!("result is: {}", res);

    //########################//
    ///////////////////////////
    //########################//

    let (_eloop, transport) =
        web3::transports::Http::new("https://testnet2.matic.network/").unwrap();

    let web3 = web3::Web3::new(transport);
    println!("~~~~~~~~~~~~~~~~~~~");
    println!("/  JUG-CLI (POC)  /");
    println!("~~~~~~~~~~~~~~~~~~~");
    
    // /**
    println!("Please enter the password for your wallet: ");
    // let str = rpassword::read_password().unwrap();
    let str = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    // println!("Your password is {}", str);

    // println!("Your password is {}", str);
    // **/

    // let mut str=String::new();

    let nodeAccount = web3.personal().new_account(&str).wait().unwrap();
    // /**
    let accounts = web3.personal().list_accounts().wait().unwrap();
    println!("Created a node account: {:?}", accounts[0]);
    // println!("Created a node account: {}", nodeAccount);
    let balance = web3.eth().balance(nodeAccount, None).wait().unwrap();
    println!("Balance of the node account: {}", balance);
    //  **/

    // let contract_address = "0x8ac900a3ea9c319d67e1dd8d355af7476fe4289c";
    // let contractd = Contract::from_json(web3.eth(), contract_address, include_bytes!("./abi.json"));

    let end_contrato: Address = "aba1309e748421a11dabd1b5a168a832a68d9b95".parse().unwrap();
    let contractd = Contract::from_json(web3.eth(), end_contrato, include_bytes!("./abi.json")).unwrap();

    // let contracta = contractd.address();
    // println!("Address of the contract: {:?}", contracta);

    // let result = contractd.call("owner", (), accounts[0], Options::default());
    let result = contractd.query("owner", (), accounts[0], Options::default(), None);
    let owner: Address = result.wait().unwrap();
    println!("Owner is : {:?}", owner);

    // let accounts = web3.eth().accounts().wait().unwrap();
    // let accounts = web3.eth().accounts().
    // syncing() .wait().unwrap();
    //  gas_price().wait().unwrap(); 
    // coinbase().wait().unwrap();
    
}
