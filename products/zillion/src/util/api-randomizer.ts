import { NetworkURL } from "./enum";

const { Random, MersenneTwister19937 } = require("random-js");
const randomJS = new Random(MersenneTwister19937.autoSeed());

/**
 * Randomize the API
 * Generate an array of random index of 1...LIST_SIZE from the api list
 * Shuffle the array and retrieve the current index
 */
export class ApiRandomizer {
    private static instance: ApiRandomizer;

    testnetIndex: number;
    mainnetIndex: number;
    randTestnetIndexList: number[];
    randMainnetIndexList: number[];
    testnetApiList: string[];
    mainnetApiList: string[];

    private constructor() {
        this.testnetIndex = 0;
        this.mainnetIndex = 0;
        this.testnetApiList = [];
        this.mainnetApiList = [];
        this.randTestnetIndexList = [];
        this.randMainnetIndexList = [];
    }

    public static getInstance(): ApiRandomizer {
        if (!ApiRandomizer.instance) {
            ApiRandomizer.instance = new ApiRandomizer()
        }
        return ApiRandomizer.instance;
    }

    /**
     * sets the correct api list and fetch a random one according to the network
     * @param url       blockchan api url to determine which network; url is from config.json => store.blockchain
     * @param apiList   list of api urls for the specific network for retry purposes; list is from config.json => store.blockchain
     * @returns         a random api from the apiList
     */
    public fetchApi(url: NetworkURL, apiList: string[]) {
        if (url === NetworkURL.MAINNET && this.mainnetApiList.length === 0) {
            this.mainnetApiList = (apiList.length > 0) ? [...apiList] : ["https://api.zilliqa.com"];
            this.randMainnetIndexList = randomJS.shuffle(Array.from(Array(this.mainnetApiList.length).keys()));
        } else if (url === NetworkURL.TESTNET && this.testnetApiList.length === 0) {
            this.testnetApiList = (apiList.length > 0) ? [...apiList] : ["https://dev-api.zilliqa.com"];
            this.randTestnetIndexList = randomJS.shuffle(Array.from(Array(this.testnetApiList.length).keys()));
        }

        let maxLen = 0;
        let randomIndex = 0;
        let api = "";

        if (url === NetworkURL.MAINNET) {
            randomIndex = this.randMainnetIndexList[this.mainnetIndex];
            api = this.mainnetApiList[randomIndex];
            maxLen = this.mainnetApiList.length;

            // prepare the index for the next try
            this.mainnetIndex = this.mainnetIndex + 1;
            if (this.mainnetIndex >= maxLen) {
                this.mainnetIndex = 0;
            }
        } else {
            randomIndex = this.randTestnetIndexList[this.testnetIndex];
            api = this.testnetApiList[randomIndex];
            maxLen = this.testnetApiList.length;

            // prepare the index for the next try
            this.testnetIndex = this.testnetIndex + 1;
            if (this.testnetIndex >= maxLen) {
                this.testnetIndex = 0;
            }
        }

        return api;
    }
}