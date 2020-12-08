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

    //let url =  'http://api.mainnet-beta.solana.com';
    let url =  'http://119.28.234.214:8899';
    //let url =  'https://devnet.solana.com';
    this.connection = new Connection(url);
    this.configPrivKey = [70,157,194,67,122,225,198,233,88,28,121,19,249,188,135,202,177,134,40,118,100,254,118,122,47,223,194,208,216,113,207,104,58,23,19,111,101,77,130,118,89,72,88,223,14,80,172,107,167,165,192,97,74,234,247,114,134,76,95,219,243,153,246,144];
    this.poolPrivKey = [0,113,195,237,177,25,179,61,16,187,69,126,120,17,128,132,129,129,223,24,75,105,203,115,46,120,43,33,129,58,224,25,195,158,112,135,218,33,117,12,1,57,48,164,246,241,113,146,209,54,220,146,42,201,175,181,254,182,109,87,56,185,120,124];
    this.payPrivKey = [140,85,119,173,23,204,204,148,203,41,107,83,176,34,167,63,180,128,189,18,187,235,122,218,79,254,216,149,117,170,115,74,56,28,173,97,136,25,66,83,199,115,122,109,206,35,28,138,109,100,88,118,102,116,122,85,208,44,64,221,40,55,226,250];

    this.payAccount = new Account(this.payPrivKey);
    this.programID = '4MmMU35rUyrrzDqcxMkGb1ycna92jdH9YoKP4RXCBwtU';
    this.configAccount = new Account(this.configPrivKey);
    this.poolAccount = new Account(this.poolPrivKey);
  }


  render() {
    return (
      <Container>


        <React.Fragment>
          <Button onClick={this.onInitialize}> initialize</Button>
        </React.Fragment>
      </Container>
    );
  }

  onInitialize() {
    let trxi = SolongLottery.createInitializeInstruction(
      this.configAccount.publicKey,
      this.poolAccount.publicKey,
      new PublicKey(this.programID),
    );

    const transaction = new Transaction();
    transaction.add(trxi);

    let signers= [this.payAccount];
    sendAndConfirmTransaction(this.connection, transaction, signers, {
        skipPreflight: false,
        commitment: 'recent',
        preflightCommitment: 'recent',
    }).then(()=>{
      console.log("done");
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