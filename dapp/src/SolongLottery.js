/**
 * @flow
 */

import { 
    PublicKey,
    SystemProgram,
    TransactionInstruction,
    SYSVAR_CLOCK_PUBKEY } from "@solana/web3.js"
import {u64} from '@solana/spl-token'
import {intFromBytes} from './utils'
import * as BufferLayout from 'buffer-layout';


/**
 * SolongLottery
 */
export class SolongLottery {
    /**
     * Construct an  Initialize instruction
     *
     */
    static createInitializeInstruction(
        adminAccountKey,
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
            {pubkey: adminAccountKey, isSigner: false, isWritable: true},
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
     * Construct an  gm instruction
     *
     */
    static createGMInstruction(
        adminAccountKey,
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
              i:3, // gm instruct 
              fund:new u64(fund).toBuffer(),
              price:new u64(price).toBuffer(),
            },
            data,
        );
      
        let keys = [
            {pubkey: adminAccountKey, isSigner: true, isWritable: true},
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
    ) {
        let resp = await connection._rpcRequest('getAccountInfo', [
            poolAccountKey.toBase58(),
            {
              encoding:'jsonParsed',
              commitment: 'recent',
            }
        ])
        let lottery = {};
        //console.log("resp:", resp);
        if (resp.result ) {
            const result = resp.result;
            //console.log("result:",result)
            const pool_buf = result.value.data[0];
            const pool = Buffer.from(pool_buf, 'base64');
            //console.log("pool:", pool);
            const award = intFromBytes(pool.slice(0,8));
            const fund = intFromBytes(pool.slice(8,16));
            const price = intFromBytes(pool.slice(16,24));
            const playerCount = intFromBytes(pool.slice(56,58));
            //console.log("Player count:", playerCount);
            let players = new Map();
            for(let i=0; i< playerCount; i++) {
                const offset = 58+i*35;
                const playerAccountKey =  new PublicKey(pool.slice(offset,offset+32)).toBase58(); 
                const playerLottery =  intFromBytes(pool.slice(offset+32,offset+32+2));
                players.set(playerAccountKey, playerLottery);
            }

            lottery = {
                award:award,
                fund:fund,
                price:price,
                players:players,
            };
            console.log("lottery:", lottery);

            return lottery; 
        } else {
            return null;
        }
    }
    
    static async GetBillboard(
        connection,
        billboardAccountKey,
    ) {
        let resp = await connection._rpcRequest('getAccountInfo', [
            billboardAccountKey.toBase58(),
            {
              encoding:'jsonParsed',
              commitment: 'recent',
            }
        ])
        let billboard = [];
        //console.log("resp:", resp);
        if (resp.result  ) {
            const result = resp.result;
            //console.log("result:",result)
            const pool_buf = result.value.data[0];
            const pool = Buffer.from(pool_buf, 'base64');
            //console.log("pool:", pool);
            
            const awardCount = intFromBytes(pool.slice(0,2));
            //console.log("awardCount count:", awardCount);
            for(let i=0; i< awardCount; i++) {
                const index = 2+(32+8+1+8)*i;
                const key =  new PublicKey(pool.slice(index,index+32)); 
                const award =  intFromBytes(pool.slice(index+32,index+40));
                const reward =  pool.slice(index+40,index+41)[0];
                const timestamp =  intFromBytes(pool.slice(index+41,index+49));
                const record = {
                    account:key.toBase58(),
                    award: award,
                    reward:reward,
                    timestamp:timestamp,
                };
                billboard.push(record)
            }
            console.log("billboard:", billboard);
            return billboard; 
        } else {
            return null;
        }
    }
}