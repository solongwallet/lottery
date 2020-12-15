
export function intFromBytes(byteArr) {
    let ret = 0;
    byteArr.forEach((val, i) => { ret += val * 256 ** i; });
    return ret;
}