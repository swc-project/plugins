export { };
Object.defineProperty(exports, "getRedis", {
    enumerable: true,
    get () {
        return getRedis;
    },
    set (v) {
        getRedis = v;
    },
    configurable: true
});
import memoize from "p-memoize";
import { getConfig } from "./config";
const getRedis = memoize(async ()=>{
    const config = await getConfig();
    return new Redis(config.redisUrl);
});
