"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
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
Object.defineProperty(exports, "__esModule", { value: true });
var turbin3_wallet_json_1 = require("../../../../../rust-project/turbin3-wallet.json");
var umi_bundle_defaults_1 = require("@metaplex-foundation/umi-bundle-defaults");
var umi_1 = require("@metaplex-foundation/umi");
var bytes_1 = require("@coral-xyz/anchor/dist/cjs/utils/bytes");
// Define our Mint address
var mint = (0, umi_1.publicKey)("HHerAXw54iWygEWVnDcRPPWQi9uCh4BVJ8ZSqYgp8n7J");
// Create a UMI connection
var umi = (0, umi_bundle_defaults_1.createUmi)('https://api.devnet.solana.com');
var keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(turbin3_wallet_json_1.default));
var signer = (0, umi_1.createSignerFromKeypair)(umi, keypair);
umi.use((0, umi_1.signerIdentity)((0, umi_1.createSignerFromKeypair)(umi, keypair)));
(function () { return __awaiter(void 0, void 0, void 0, function () {
    var encode;
    return __generator(this, function (_a) {
        try {
            encode = bytes_1.bs58.encode([76, 52, 55, 154, 46, 78, 56, 105, 88, 166, 215, 144, 29, 146, 121, 202, 59, 129, 216, 16, 248, 130, 207, 122, 29, 225, 103, 209, 14, 151, 200, 153, 153, 254, 93, 52, 214, 208, 130, 119, 26, 64, 36, 167, 16, 13, 242, 237, 178, 21, 200, 28, 64, 130, 176, 130, 171, 162, 39, 66, 171, 236, 171, 187]);
            console.log('encode: ', encode);
        }
        catch (e) {
            console.error("Oops, something went wrong: ".concat(e));
        }
        return [2 /*return*/];
    });
}); })();
