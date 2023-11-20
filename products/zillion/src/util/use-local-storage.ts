import { convertNetworkUrlToLabel } from "./utils";

/**
 * https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API/Using_the_Web_Storage_API#Feature-detecting_localStorage
 * checks if the browser accepts local storage
 */
export function isStorageAvailable(type: string) {
    var storage;
    try {
        storage = (window as any)[type];
        var x = '__storage_test__';
        storage.setItem(x, x);
        storage.removeItem(x);
        return true;
    } catch (e) {
        // DOM exception
        console.error('no local storage present');
        return false;
    }
}

/*
 * store item in local storage

 * <wallet_address> : {
 *     <network>: {
 *         key1: value,
 *         key2: value
 *     }
 * }
 * 
 * wallet: bech32 address
 * proxy: checksum address
 * network: blockchain url will convert to label later
 * key: storage key
 * value: storage value
 * 
*/
export function storeLocalItem(wallet: string, proxy: string, network: string, key: string, value: any) {
    
    if (!isStorageAvailable("localStorage")) {
        return null;
    }

    network = convertNetworkUrlToLabel(network);
    const storedValue: any = window.localStorage.getItem(wallet);
    let currStorageMap: any;

    if (proxy === null || proxy === "") {
        proxy = "proxy";
    } else {
        proxy = proxy.substring(0, 10);
    }
    const accessKey = network + "-" + proxy;

    if (storedValue !== null) {
        let localStorageMap = JSON.parse(storedValue);
        
        if (localStorageMap.hasOwnProperty(accessKey) && key in localStorageMap[accessKey]) {
            // update the respective section only
            localStorageMap[accessKey][key] = value;

        } else if (!localStorageMap.hasOwnProperty(accessKey)) {
            // first time this particular wallet address is storing on this network
            localStorageMap[accessKey] = {};
            localStorageMap[accessKey][key] = value;

        } else {
            // first time this particular wallet address is storing this value
            localStorageMap = {};
            localStorageMap[accessKey] = {};
            localStorageMap[accessKey][key] = value; 
        }
        currStorageMap = localStorageMap;

    } else {
        currStorageMap = {};
        currStorageMap[accessKey] = {};
        currStorageMap[accessKey][key] = value;
    }

    window.localStorage.setItem(wallet, JSON.stringify(currStorageMap));
}

/*
 * retrieve item from local storage if exists
 * returns defaultValue if item does not exist
 * 
 * wallet: bech32 address
 * proxy: checksum address
 * network: blockchain url will convert to label later
 * key: storage key
*/
export function getLocalItem(wallet: string, proxy: string, network: string, key: string, defaultValue: any) {
    if (!isStorageAvailable("localStorage")) {
        return defaultValue;
    }

    network = convertNetworkUrlToLabel(network);
    const storedValue: any = window.localStorage.getItem(wallet);

    if (proxy === null || proxy === "") {
        proxy = "proxy";
    } else {
        proxy = proxy.substring(0, 10);
    }
    const accessKey = network + "-" + proxy;

    if (storedValue !== null) {
        const localStorageMap = JSON.parse(storedValue);
        if (localStorageMap.hasOwnProperty(accessKey) && key in localStorageMap[accessKey]) {
            return localStorageMap[accessKey][key];
        }
    }
    return defaultValue;
}