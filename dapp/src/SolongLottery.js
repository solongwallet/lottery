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
import * as Layout from './Layout';
import {intFromBytes} from './utils'


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
        adminAccountKey,
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
            {pubkey: adminAccountKey, isSigner: true, isWritable: true},
            {pubkey: playerAccountKey, isSigner: false, isWritable: true},
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

    static async GetLotteryPool(
        connection,
        poolAccountKey,
        programID,
    ) {
        let resp = await connection._rpcRequest('getProgramAccounts', [
            programID.toBase58(),
            {
              encoding:'jsonParsed',
              commitment: 'recent',
                filters:[{"dataSize": Layout.poolSpace}]
            }
        ])
        let lottery = {};
        if (resp.result && resp.result.length > 0 ) {
            resp.result.forEach( result =>{
                const pubkey = result.pubkey;
                if (pubkey ==  poolAccountKey.toBase58()) {
                    console.log("result:",result)
                    const pool_buf = result.account.data[0];
                    const pool = Buffer.from(pool_buf, 'base64');
                    console.log("pool:", pool);
                    const award = intFromBytes(pool.slice(0,8));
                    const fund = intFromBytes(pool.slice(8,16));
                    const price = intFromBytes(pool.slice(16,24));
                    const feeAccountKey = new PublicKey(pool.slice(24,56)).toBase58();
                    const playerCount = intFromBytes(pool.slice(88,90));
                    console.log("Player count:", playerCount);
                    let players = new Map();
                    for(let i=0; i< playerCount; i++) {
                        const playerAccountKey =  new PublicKey(pool.slice(90,122)).toBase58(); 
                        const playerLottery =  intFromBytes(pool.slice(122,124));
                        players.set(playerAccountKey, playerLottery);
                    }

                    lottery = {
                        award:award,
                        fund:fund,
                        price:price,
                        feeAccountKey:feeAccountKey,
                        players:players,
                    };
                    console.log("lottery:", lottery);
                }
            });
            return lottery; 
        } else {
            return null;
        }
    }
    
    static async GetBillboard(
        connection,
        billboardAccountKey,
        programID,
    ) {
        let resp = await connection._rpcRequest('getProgramAccounts', [
            programID.toBase58(),
            {
              encoding:'jsonParsed',
              commitment: 'recent',
                filters:[{"dataSize": Layout.awardSpace}]
            }
        ])
        let billboard = [];
        if (resp.result && resp.result.length > 0 ) {
            resp.result.forEach( result =>{
                const pubkey = result.pubkey;
                if (pubkey ==  billboardAccountKey.toBase58()) {
                    console.log("result:",result)
                    const pool_buf = result.account.data[0];
                    const pool = Buffer.from(pool_buf, 'base64');
                    console.log("pool:", pool);
                    
                    const awardCount = intFromBytes(pool.slice(0,2));
                    console.log("awardCount count:", awardCount);
                    for(let i=0; i< awardCount; i++) {
                        const index = (32+9)*i;
                        const key =  new PublicKey(pool.slice(index,index+32)); 
                        const award =  intFromBytes(pool.slice(index+32,index+40));
                        const reward =  pool.slice(index+40,index+41)[0];
                        const record = {
                            account:key.toBase58(),
                            award: award,
                            reward:reward,
                        };
                        billboard.push(record)
                    }


                    console.log("billboard:", billboard);
                }
            });
            return billboard; 
        } else {
            return null;
        }
    }
}