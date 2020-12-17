// @flow

import * as BufferLayout from 'buffer-layout';

/**
 * Layout for a public key
 */
export function publicKey(property) {
    return BufferLayout.blob(32, property);
}

export const poolSpace = 8+8+8+32+32+2+1000*(32+2);
export const awardSpace = 1000*(32+9);

/**
 * Layout for LotteryInitState 
 */

export const LotteryInitState = BufferLayout.struct([

]);