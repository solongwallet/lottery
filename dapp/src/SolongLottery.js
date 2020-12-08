/**
 * @flow
 */

import * as BufferLayout from 'buffer-layout';
import { Connection,
    Transaction,
    PublicKey,
    SystemProgram,
    TransactionInstruction,
    sendAndConfirmTransaction,
    Account} from "@solana/web3.js"
import bs58 from 'bs58';
import {u64} from '@solana/spl-token'
import { uint64 } from 'solong.js/src/token';


/**
 * SolongLottery
 */
export class SolongLottery {
    /**
     * Construct an  Initialize instruction
     *
     */
    static createInitializeInstruction(
        configAccountKey,
        poolAccountKey,
        programID,
    ) {

        const dataLayout = BufferLayout.struct([
            BufferLayout.u8("i"),
            BufferLayout.blob(8,"supply"),
        ]);
      
        const data = Buffer.alloc(dataLayout.span);
        dataLayout.encode(
            {
              i:1, // initialize instruct 
              supply:new u64("0").toBuffer(),
            },
            data,
        );
      
        let keys = [
            {pubkey: configAccountKey, isSigner: false, isWritable: true},
            {pubkey: poolAccountKey, isSigner: false, isWritable: true},
            {pubkey: programID, isSigner: false, isWritable: true},
        ];

        const  trxi = new TransactionInstruction({
            keys,
            programId: programID,
            data,
        });
        return trxi;
    }
}