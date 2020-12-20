
const lottery = require("@solong/lottery.js");
const solana  = require("@solana/web3.js");

function main() {
    //let url =  'http://api.mainnet-beta.solana.com';
    //let url =  'http://119.28.234.214:8899';
    let url =  'https://devnet.solana.com';
    let connection = new solana.Connection(url);
    let programID = new solana.PublicKey('3y8sNACRdCXRjCsJwuKhrge6Ftkr6okxbpZpFgEYjo7o');
    let adminPrivKey = [140,85,119,173,23,204,204,148,203,41,107,83,176,34,167,63,180,128,189,18,187,235,122,218,79,254,216,149,117,170,115,74,56,28,173,97,136,25,66,83,199,115,122,109,206,35,28,138,109,100,88,118,102,116,122,85,208,44,64,221,40,55,226,250];
    let playerPrivKey = [136,110,52,25,177,59,33,252,208,157,67,200,66,34,83,248,94,110,161,40,156,235,104,28,73,233,3,255,109,59,85,164,240,29,177,212,46,230,9,255,12,214,10,209,78,79,174,119,160,91,178,114,42,99,0,177,50,110,54,221,212,219,204,115];

    let adminAccount = new solana.Account(this.adminPrivKey);
    let playerAccount = new solana.Account(this.playerPrivKey);
    let billboard = new solana.PublicKey("GiTauptuEQThbuS9HbJVLmQinXx1WyS9xHzUemAs49pz");
    let pool = new solana.PublicKey("5TdmTJcSM6NVUXnE7o13kwk8diEXFRwTgZWx4x1sVURb");
    
    let trxi = lottery.SolongLottery.createSignInstruction(
        playerAccount.publicKey,
        pool,
        programID,
      );
  
    let transaction = new solana.Transaction();
    transaction.add(trxi);
  
    let signers= [playerAccount];
    solana.sendAndConfirmTransaction(connection, transaction, signers, {
          skipPreflight: false,
          commitment: 'recent',
          preflightCommitment: 'recent',
      }).then(()=>{
        console.log("done sign");
      }).catch((e)=>{
        console.log("error:", e);
      }) 
}

main();