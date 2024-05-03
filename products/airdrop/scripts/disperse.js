const hre = require("hardhat");

const { fromBech32Address, toBech32Address } = require("@zilliqa-js/crypto");
const { validation } = require("@zilliqa-js/util");
const { open } = require("node:fs/promises");

async function main() {

  const disperse = await ethers.getContractAt("Disperse", "0x38048F4B71a87a31d21C86FF373a91d1E401bea5");
  const token = await ethers.getContractAt("ERC20", "0xf01f7FF8E38759707eE4167f0db48694677D15ad");
  const batch = 100;

  const decimals = await token.decimals();
  const multiplier = BigInt(10) ** decimals;

  const recipients = [];
  const amounts = [];
  var total = BigInt(0);
  const file = await open("./scripts/input.csv");
  for await (const line of file.readLines()) {
	  const [address, amountStr] = line.split(",");
	  const recipient = address && validation.isBech32(address) ? fromBech32Address(address) : address;
	  recipients.push(recipient.toLowerCase());
    const amount = BigInt(amountStr);
	  amounts.push(amount * multiplier);
    total += amount * multiplier;
    //console.log(recipient, amount);
  }
  
  txn = await token.approve(disperse, total);
  rcpt = await txn.wait();

  for (start = 0; start < recipients.length; start += batch) {
	  end = start + batch < recipients.length ? start + batch : recipients.length;
	  txn = await disperse.disperseToken(token, recipients.slice(start, end), amounts.slice(start, end));
    rcpt = await txn.wait();
	  console.log(txn.hash, start, end);
  }
  
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
