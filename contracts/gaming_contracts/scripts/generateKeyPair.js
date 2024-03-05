const elliptic = require("elliptic");
const ec = new elliptic.ec("secp256k1");
const log = console.log;

const generate = async () => {
  try {
    log("Generating key pair");
    let keyPair = ec.genKeyPair();
    let privKey = keyPair.getPrivate("hex");
    let pubKey = keyPair.getPublic();
    log(`Private key: ${privKey}`);
    log("Public key :", pubKey.encode("hex").substr(2));
    log("Public key (compressed):", pubKey.encodeCompressed("hex"));
  } catch (err) {
    log("Has error");
    log(err);
    return { has_error: true, data: err };
  }
};

generate();
