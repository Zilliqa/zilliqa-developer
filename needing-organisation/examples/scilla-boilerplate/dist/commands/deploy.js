#!/usr/bin/env node
"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : new P(function (resolve) { resolve(result.value); }).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var _this = this;
Object.defineProperty(exports, "__esModule", { value: true });
var chalk = require("chalk");
var shell = require("shelljs");
var Listr = require("listr");
var yargs = require("yargs");
var zilliqa_testing_library_1 = require("zilliqa-testing-library");
var fs = require("fs");
var path = require("path");
var CURR_DIR = process.cwd();
function postProcessNode() {
    return __awaiter(this, void 0, void 0, function () {
        var config, ZT, contractCode, contract;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    config = require(CURR_DIR + '/zilliqa.config.js');
                    ZT = new zilliqa_testing_library_1.default({ network: config.networkUrl });
                    // Import the wallet from PRIVATE KEY that wa provided
                    return [4 /*yield*/, ZT.importAccounts([config.accountPrivateKey])];
                case 1:
                    // Import the wallet from PRIVATE KEY that wa provided
                    _a.sent();
                    contractCode = fs.readFileSync(path.join(CURR_DIR, config.contractFile), "utf8");
                    return [4 /*yield*/, ZT.loadContract(contractCode)];
                case 2:
                    contract = _a.sent();
                    if (!(contract !== undefined && contract.deploy !== undefined)) return [3 /*break*/, 4];
                    return [4 /*yield*/, contract.deploy(ZT.accounts[0].address, config.init)];
                case 3: 
                // helloWorld.deploy return a tuple containing transaction object and a contract object
                return [2 /*return*/, _a.sent()];
                case 4: throw new Error('Contract could not be imported. Maybe scilla-checker failed');
            }
        });
    });
}
var deploy = function () {
    var tasks = new Listr([
        {
            title: 'Run tests',
            task: function () { return __awaiter(_this, void 0, void 0, function () { return __generator(this, function (_a) {
                return [2 /*return*/, shell.exec('mocha --timeout=10000')];
            }); }); }
        },
        {
            title: 'Deploy contract',
            task: function (ctx) { return __awaiter(_this, void 0, void 0, function () {
                var response;
                return __generator(this, function (_a) {
                    switch (_a.label) {
                        case 0: return [4 /*yield*/, postProcessNode()];
                        case 1:
                            response = _a.sent();
                            ctx.tx = response[0];
                            ctx.contract = response[1];
                            return [2 /*return*/];
                    }
                });
            }); }
        }
    ]);
    tasks.run().then(function (ctx) {
        console.log('\nTransaction receipt: ', ctx.tx.receipt, '\n');
        console.log('Contract address: ', chalk.yellow(ctx.contract.address), '\n\n');
        if (yargs.argv.verbose && yargs.argv.verbose === true) {
            console.log('Init params: ', ctx.contract.init);
        }
    }).catch(function (err) {
        console.error(err);
    });
};
exports.default = deploy;
//# sourceMappingURL=deploy.js.map