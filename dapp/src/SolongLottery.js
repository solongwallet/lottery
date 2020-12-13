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
    SYSVAR_CLOCK_PUBKEY,
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
        ownerAccountKey,
        feeAccountKey,
        billboardAccountKey,
        poolAccountKey,
        programID,
        fund,
        price
    ) {

        const dataLayout = BufferLayout.struct([
            BufferLayout.u8("i"),
            BufferLayout.blob(8,"fund"),
            BufferLayout.blob(8,"price"),
        ]);
      
        const data = Buffer.alloc(dataLayout.span);
        dataLayout.encode(
            {
              i:1, // initialize instruct 
              fund:new u64(fund).toBuffer(),
              price:new u64(price).toBuffer(),
            },
            data,
        );
      
        let keys = [
            {pubkey: ownerAccountKey, isSigner: false, isWritable: true},
            {pubkey: feeAccountKey, isSigner: false, isWritable: true},
            {pubkey: poolAccountKey, isSigner: false, isWritable: true},
            {pubkey: billboardAccountKey, isSigner: false, isWritable: true},
        ];

        const  trxi = new TransactionInstruction({
            keys,
            programId: programID,
            data,
        });
        return trxi;
    }

    /**
     * Construct an  Sign instruction
     *
     */
    static createSignInstruction(
        playerAccountKey,
        poolAccountKey,
        programID,
    ) {

        const dataLayout = BufferLayout.struct([
            BufferLayout.u8("i"),
        ]);
      
        const data = Buffer.alloc(dataLayout.span);
        dataLayout.encode(
            {
              i:2, // sign instruct 
            },
            data,
        );
      
        let keys = [
            {pubkey: playerAccountKey, isSigner: true, isWritable: true},
            {pubkey: poolAccountKey, isSigner: false, isWritable: true},
        ];

        const  trxi = new TransactionInstruction({
            keys,
            programId: programID,
            data,
        });
        return trxi;
    }

    /**
     * Construct an  buy instruction
     *
     */
    static createBuyInstruction(
        playerAccountKey,
        feeAccountKey,
        poolAccountKey,
        programID,
    ) {

        const dataLayout = BufferLayout.struct([
            BufferLayout.u8("i"),
        ]);
      
        const data = Buffer.alloc(dataLayout.span);
        dataLayout.encode(
            {
              i:3, // buy instruct 
            },
            data,
        );
      
        let keys = [
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: true},
            {pubkey: playerAccountKey, isSigner: true, isWritable: true},
            {pubkey: feeAccountKey, isSigner: false, isWritable: true},
            {pubkey: poolAccountKey, isSigner: false, isWritable: true},
        ];

        const  trxi = new TransactionInstruction({
            keys,
            programId: programID,
            data,
        });
        return trxi;
    }

    /**
     * Construct an  roll instruction
     *
     */
    static createRollInstruction(
        adminAccountKey,
        poolAccountKey,
        billboardAccountKey,
        programID,
    ) {

        const dataLayout = BufferLayout.struct([
            BufferLayout.u8("i"),
        ]);
      
        const data = Buffer.alloc(dataLayout.span);
        dataLayout.encode(
            {
              i:4, // roll instruct 
            },
            data,
        );
      
        let keys = [
            {pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: true},
            {pubkey: adminAccountKey, isSigner: true, isWritable: true},
            {pubkey: poolAccountKey, isSigner: false, isWritable: true},
            {pubkey: billboardAccountKey, isSigner: false, isWritable: true},
        ];

        const  trxi = new TransactionInstruction({
            keys,
            programId: programID,
            data,
        });
        return trxi;
    }



    /**
     * Construct an  reward instruction
     *
     */
    static createRewardInstruction(
        playerAccountKey,
        billboardAccountKey,
        programID,
    ) {

        const dataLayout = BufferLayout.struct([
            BufferLayout.u8("i"),
        ]);
      
        const data = Buffer.alloc(dataLayout.span);
        dataLayout.encode(
            {
              i:5, // reward instruct 
            },
            data,
        );
      
        let keys = [
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: true},
            {pubkey: playerAccountKey, isSigner: true, isWritable: true},
            {pubkey: billboardAccountKey, isSigner: false, isWritable: true},
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