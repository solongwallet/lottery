import React from 'react';
import logo from './logo.svg';
import './App.css';
import TextField from '@material-ui/core/TextField'
import Container from '@material-ui/core/Box'
import Divider from '@material-ui/core/Divider'

import {AccountManager} from 'solong.js'
import { LAMPORTS_PER_SOL,Account, PublicKey, Connection, SystemProgram ,Transaction,sendAndConfirmTransaction} from '@solana/web3.js';
import { Button,Grid } from '@material-ui/core';
import {SolongLottery} from './SolongLottery.js/SolongLottery';
import * as Layout from './SolongLottery.js/Layout';


class Content extends React.Component {

  constructor(props) {
    super(props)
    this.state = { };
    this.onInitialize = this.onInitialize.bind(this);
    this.onSign = this.onSign.bind(this);
    this.onGM= this.onGM.bind(this);
    this.onRoll = this.onRoll.bind(this);
    this.onReward = this.onReward.bind(this);
    this.onQueryBillboard = this.onQueryBillboard.bind(this);
    this.onQueryPool = this.onQueryPool.bind(this);

    //let url =  'http://api.mainnet-beta.solana.com';
    //let url =  'https://solana-api.projectserum.com';
    let url =  'http://119.28.234.214:8899';
    //let url =  'https://devnet.solana.com';
    this.connection = new Connection(url);
    this.adminPrivKey = [140,85,119,173,23,204,204,148,203,41,107,83,176,34,167,63,180,128,189,18,187,235,122,218,79,254,216,149,117,170,115,74,56,28,173,97,136,25,66,83,199,115,122,109,206,35,28,138,109,100,88,118,102,116,122,85,208,44,64,221,40,55,226,250];
    this.playerPrivKey = [136,110,52,25,177,59,33,252,208,157,67,200,66,34,83,248,94,110,161,40,156,235,104,28,73,233,3,255,109,59,85,164,240,29,177,212,46,230,9,255,12,214,10,209,78,79,174,119,160,91,178,114,42,99,0,177,50,110,54,221,212,219,204,115];

    this.adminAccount = new Account(this.adminPrivKey);
    this.playerAccount = new Account(this.playerPrivKey);
    this.programID = new PublicKey('95zvnioz4A2wc7cezVf9SPTmDxAYcVZAAzjdnU5jDpTR');
    this.billboardAccount = new Account();
    this.poolAccount = new Account(); 
    this.billboardAccountKey = new PublicKey("Dhk5ucvtyWXCFXtqwgNNWEyT7NHdzfyNfYGkJHMCx5ZR");
    this.poolAccountKey = new PublicKey("GgtPHBhAWpJMxmWLcWhA5wY9h3NWkCZN2kGYeXAPUePG"); 
  }


  render() {
    return (
      <Container>


        <React.Fragment>
          <Button onClick={this.onInitialize}> initialize</Button>
        </React.Fragment>
        <Divider />
        <React.Fragment>
          <Button onClick={this.onSign}> sign</Button>
        </React.Fragment>
        <Divider />
        <React.Fragment>
          <Button onClick={this.onGM}> GM</Button>
        </React.Fragment>
        <Divider />
        <React.Fragment>
          <Button onClick={this.onRoll}> roll</Button>
        </React.Fragment>
        <Divider />
        <React.Fragment>
          <Button onClick={this.onReward}> reward</Button>
        </React.Fragment>
        <Divider />
        <React.Fragment>
          <Button onClick={this.onQueryPool}> Pool </Button>
        </React.Fragment>
        <Divider />
        <React.Fragment>
          <Button onClick={this.onQueryBillboard}> billboard</Button>
        </React.Fragment>
      </Container>
    );
  }

  async onQueryPool() {
    SolongLottery.GetLotteryPool(this.connection, 
      this.poolAccountKey).then((pool)=>{
          console.log("pool:", pool);
      });
  }

  async onQueryBillboard() {
    SolongLottery.GetBillboard(this.connection, 
      //this.billboardAccountKey,
      this.billboardAccount.publicKey).then((pool)=>{
          console.log("award:", pool);
      });
  }

  async onRoll() {
    let trxi = SolongLottery.createRollInstruction(
      this.adminAccount.publicKey,
      //this.poolAccountKey,
      this.poolAccount.publicKey,
      //this.billboardAccountKey,
      this.billboardAccount.publicKey,
      this.programID,
    );

    const transaction = new Transaction();
    transaction.add(trxi);

    let signers= [this.adminAccount];
    sendAndConfirmTransaction(this.connection, transaction, signers, {
        skipPreflight: false,
        commitment: 'recent',
        preflightCommitment: 'recent',
    }).then(()=>{
      console.log("done roll");
    }).catch((e)=>{
      console.log("error:", e);
    }) 
  }

  async onReward() {
    let trxi = SolongLottery.createRewardInstruction(
      this.adminAccount.publicKey,
      this.playerAccount.publicKey,
      //this.billboardAccountKey,
      this.billboardAccount.publicKey,
      this.programID,
    );

    const transaction = new Transaction();
    transaction.add(trxi);

    let signers= [this.adminAccount];
    sendAndConfirmTransaction(this.connection, transaction, signers, {
        skipPreflight: false,
        commitment: 'recent',
        preflightCommitment: 'recent',
    }).then(()=>{
      console.log("done reward");
    }).catch((e)=>{
      console.log("error:", e);
    }) 
  }


  async onSign() {

    for (let i=0;i<5;i++) {

      let player = new Account();
      this.connection.requestAirdrop(player.publicKey, 10*1000000000).then(()=>{
        setTimeout(
          ()=>{
// timeout
            console.log("player:", player.publicKey.toBase58())
        let trxi = SolongLottery.createSignInstruction(
          //this.playerAccount.publicKey,
          //this.poolAccountKey,
          player.publicKey,
          this.poolAccount.publicKey,
          this.programID,
        );
  
        const transaction = new Transaction();
        transaction.add(trxi);
  
        let signers= [player];
        sendAndConfirmTransaction(this.connection, transaction, signers, {
            skipPreflight: false,
            commitment: 'recent',
            preflightCommitment: 'recent',
        }).then(()=>{
          console.log("done sign");
        }).catch((e)=>{
          console.log("error:", e);
        }) 
// timeout
          },3000
        )
      })

    }
  }

  async onGM() {
    let trxi = SolongLottery.createGMInstruction(
      this.adminAccount.publicKey,
      //this.poolAccountKey,
      this.poolAccount.publicKey,
      this.programID,
      2000000000,
      2000000000,
    );

    const transaction = new Transaction();
    transaction.add(trxi);

    let signers= [this.adminAccount];
    sendAndConfirmTransaction(this.connection, transaction, signers, {
        skipPreflight: false,
        commitment: 'recent',
        preflightCommitment: 'recent',
    }).then(()=>{
      console.log("done GM");
    }).catch((e)=>{
      console.log("error:", e);
    }) 
  }

  async onInitialize() {
    //return null;
    let poolSpace = Layout.poolSpace;
    let awardSpace = Layout.awardSpace;
    let poolNeeded = await this.connection.getMinimumBalanceForRentExemption(poolSpace);
    let awardNeeded = await this.connection.getMinimumBalanceForRentExemption(awardSpace);

    const trxi0 =  SystemProgram.createAccount({
      fromPubkey: this.adminAccount.publicKey,
      newAccountPubkey: this.billboardAccount.publicKey,
      lamports: awardNeeded,
      space: awardSpace,
      programId: this.programID,
    });
    console.log("award:", this.billboardAccount.publicKey.toBase58());

    const trxi1 =  SystemProgram.createAccount({
      fromPubkey: this.adminAccount.publicKey,
      newAccountPubkey: this.poolAccount.publicKey,
      lamports: poolNeeded,
      space: poolSpace,
      programId: this.programID,
    });
    console.log("pool:", this.poolAccount.publicKey.toBase58());


    let trxi = SolongLottery.createInitializeInstruction(
      this.adminAccount.publicKey,
      //this.billboardAccount.publicKey,
      this.billboardAccountKey,
      this.poolAccountKey,
      //this.poolAccount.publicKey,
      this.programID,
      1000000000,
      1000000000,
    );

    const transaction = new Transaction();
    //transaction.add(trxi0);
    //transaction.add(trxi1);
    transaction.add(trxi);

    //let signers= [this.adminAccount, this.billboardAccount, this.poolAccount];
    let signers= [this.adminAccount];
    sendAndConfirmTransaction(this.connection, transaction, signers, {
        skipPreflight: false,
        commitment: 'recent',
        preflightCommitment: 'recent',
    }).then(()=>{
      console.log("done init");
    }).catch((e)=>{
      console.log("error:", e);
    })


  }

}


function App() {
  return (
    <Content />
  );
}

export default App;