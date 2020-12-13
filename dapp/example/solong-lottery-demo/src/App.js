import React from 'react';
import logo from './logo.svg';
import './App.css';
import TextField from '@material-ui/core/TextField'
import Container from '@material-ui/core/Box'
import Divider from '@material-ui/core/Divider'

import {AccountManager} from 'solong.js'
import { LAMPORTS_PER_SOL,Account, PublicKey, Connection, SystemProgram ,Transaction,sendAndConfirmTransaction} from '@solana/web3.js';
import { Button,Grid } from '@material-ui/core';
import {SolongLottery} from './SolongLottery.js/SolongLottery'


class Content extends React.Component {

  constructor(props) {
    super(props)
    this.state = { };
    this.onInitialize = this.onInitialize.bind(this);
    this.onSign = this.onSign.bind(this);
    this.onBuy = this.onBuy.bind(this);
    this.onRoll = this.onRoll.bind(this);
    this.onReward = this.onReward.bind(this);

    //let url =  'http://api.mainnet-beta.solana.com';
    let url =  'http://119.28.234.214:8899';
    //let url =  'https://devnet.solana.com';
    this.connection = new Connection(url);
    this.payPrivKey = [140,85,119,173,23,204,204,148,203,41,107,83,176,34,167,63,180,128,189,18,187,235,122,218,79,254,216,149,117,170,115,74,56,28,173,97,136,25,66,83,199,115,122,109,206,35,28,138,109,100,88,118,102,116,122,85,208,44,64,221,40,55,226,250];

    this.payAccount = new Account(this.payPrivKey);
    this.programID = new PublicKey('FKZUNWiiE5oAmnPcdC8CQPgnmj3MxEtqML1PqfQBjnAy');
    this.billboardAccount = new Account();
    this.poolAccount = new Account(); 
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
          <Button onClick={this.onBuy}> buy</Button>
        </React.Fragment>
        <React.Fragment>
          <Button onClick={this.onRoll}> roll</Button>
        </React.Fragment>
        <React.Fragment>
          <Button onClick={this.onReward}> reward</Button>
        </React.Fragment>
      </Container>
    );
  }

  async onRoll() {
    let trxi = SolongLottery.createRollInstruction(
      this.payAccount.publicKey,
      this.poolAccount.publicKey,
      this.billboardAccount.publicKey,
      this.programID,
    );

    const transaction = new Transaction();
    transaction.add(trxi);

    let signers= [this.payAccount];
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
      this.payAccount.publicKey,
      this.billboardAccount.publicKey,
      this.programID,
    );

    const transaction = new Transaction();
    transaction.add(trxi);

    let signers= [this.payAccount];
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
    let trxi = SolongLottery.createSignInstruction(
      this.payAccount.publicKey,
      this.poolAccount.publicKey,
      this.programID,
    );

    const transaction = new Transaction();
    transaction.add(trxi);

    let signers= [this.payAccount];
    sendAndConfirmTransaction(this.connection, transaction, signers, {
        skipPreflight: false,
        commitment: 'recent',
        preflightCommitment: 'recent',
    }).then(()=>{
      console.log("done sign");
    }).catch((e)=>{
      console.log("error:", e);
    }) 
  }

  async onBuy() {
    let trxi = SolongLottery.createBuyInstruction(
      this.payAccount.publicKey,
      this.payAccount.publicKey,
      this.poolAccount.publicKey,
      this.programID,
    );

    const transaction = new Transaction();
    transaction.add(trxi);

    let signers= [this.payAccount];
    sendAndConfirmTransaction(this.connection, transaction, signers, {
        skipPreflight: false,
        commitment: 'recent',
        preflightCommitment: 'recent',
    }).then(()=>{
      console.log("done buy");
    }).catch((e)=>{
      console.log("error:", e);
    }) 
  }

  async onInitialize() {
    let poolSpace = 8+32+2+1000*(32+2);
    let awardSpace = 1000*(32+13);
    let poolNeeded = await this.connection.getMinimumBalanceForRentExemption(poolSpace);
    let awardNeeded = await this.connection.getMinimumBalanceForRentExemption(awardSpace);

    const trxi0 =  SystemProgram.createAccount({
      fromPubkey: this.payAccount.publicKey,
      newAccountPubkey: this.billboardAccount.publicKey,
      lamports: awardNeeded,
      space: awardSpace,
      programId: this.programID,
    });
    console.log("award:", this.billboardAccount.publicKey.toBase58());

    const trxi1 =  SystemProgram.createAccount({
      fromPubkey: this.payAccount.publicKey,
      newAccountPubkey: this.poolAccount.publicKey,
      lamports: poolNeeded,
      space: poolSpace,
      programId: this.programID,
    });
    console.log("pool:", this.poolAccount.publicKey.toBase58());


    let trxi = SolongLottery.createInitializeInstruction(
      this.payAccount.publicKey,
      this.payAccount.publicKey,
      this.billboardAccount.publicKey,
      this.poolAccount.publicKey,
      this.programID,
      10000000000,
      1000000000,
    );

    const transaction = new Transaction();
    transaction.add(trxi0);
    transaction.add(trxi1);
    transaction.add(trxi);

    let signers= [this.payAccount, this.billboardAccount, this.poolAccount];
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