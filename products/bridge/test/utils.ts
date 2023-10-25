import { expect } from "chai";
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { createProvider } from "hardhat/internal/core/providers/construction";
import { HardhatEthersProvider } from "@nomicfoundation/hardhat-ethers/internal/hardhat-ethers-provider";
import hre from "hardhat";
import { ethers } from "hardhat";

var default_name;
var default_config;
var default_provider;
var default_ethers_provider;

export async function switchNetwork(id = 0) {
  if (!default_name) {
    default_name = hre.network.name;
    default_config = hre.network.config;
    default_provider = hre.network.provider;
    default_ethers_provider = hre.ethers.provider;
  }
  if (id > 0) {
    hre.network.name = "net" + id;
    hre.network.config = hre.config.networks[hre.network.name];
    hre.network.provider = await createProvider(
      hre.config,
      hre.network.name,
      hre.artifacts
    );
    hre.ethers.provider = new HardhatEthersProvider(
      hre.network.provider,
      hre.network.name
    );
  } else {
    hre.network.name = default_name;
    hre.network.config = default_config;
    hre.network.provider = default_provider;
    hre.ethers.provider = default_ethers_provider;
  }
}

export async function obtainCalls(validators, relayer) {
  // validators see the Relayed event
  const index = Math.floor(Math.random() * validators.length);
  const blockNum = await ethers.provider.getBlockNumber();
  const logs = await validators[index].provider.getLogs({
    fromBlock: blockNum - 100,
    toBlock: blockNum,
    address: await relayer.getAddress(),
    topics: [ethers.id("Relayed(address,address,bytes,bool,bytes4,uint256)")],
  });
  const calls = [];
  for (let i = 0; i < logs.length; i++) {
    const [caller, callee, call, readonly, callback, nonce] =
      ethers.AbiCoder.defaultAbiCoder().decode(
        ["address", "address", "bytes", "bool", "bytes4", "uint256"],
        logs[i].data
      );
    calls.push({ caller, callee, call, readonly, callback, nonce });
  }
  return calls;
}

export async function confirmCall(
  validators,
  relayer,
  caller,
  callee,
  call,
  readonly,
  callback,
  nonce
) {
  // validators sign the hash of the Relayed event data and submit their signature
  const message = ethers.AbiCoder.defaultAbiCoder().encode(
    ["address", "address", "bytes", "bool", "bytes4", "uint256"],
    [caller, callee, call, readonly, callback, nonce]
  );
  const hash = ethers.hashMessage(ethers.getBytes(message));
  const signatures = [];
  const supermajority = Math.floor((validators.length * 2) / 3) + 1;
  const signerIndices = [...Array(validators.length).keys()]
    .map((a) => [a, Math.random()])
    .sort((a, b) => (a[1] < b[1] ? -1 : 1))
    .slice(0, supermajority)
    .map((a) => a[0])
    .sort((a, b) => ethers.toNumber(a) - ethers.toNumber(b));
  for (const index of signerIndices) {
    signatures.push(
      await validators[index].signMessage(ethers.getBytes(message))
    );
    const tx = await relayer
      .connect(validators[index])
      .echo(hash, index, signatures.slice(-1)[0]);
    const rcpt = await tx.wait();
    await expect(tx)
      .to.emit(relayer, "Echoed")
      .withArgs(hash, index, signatures.slice(-1)[0]);
  }

  // validators retrieve the signatures submitted by other validators from the Echoed events
  for (const index of signerIndices) {
    const blockNum = await ethers.provider.getBlockNumber();
    const logs = await validators[index].provider.getLogs({
      fromBlock: blockNum - 100,
      toBlock: blockNum,
      address: await relayer.getAddress(),
      topics: [ethers.id("Echoed(bytes32,uint16,bytes)"), hash],
    });

    for (let i = 0; i < logs.length; i++) {
      const res = ethers.AbiCoder.defaultAbiCoder().decode(
        ["uint16", "bytes"],
        logs[i].data
      );
      expect(res[0]).to.equal(signerIndices[i]);
      expect(res[1]).to.equal(signatures[i]);
    }
  }
  return { signerIndices, signatures };
}

export async function queryCall(validators, relayer, caller, callee, call) {
  // the supermajority of the validators retrieves the result using a view call
  const supermajority = Math.floor((validators.length * 2) / 3) + 1;
  const sample = [...Array(validators.length).keys()]
    .map((a) => [a, Math.random()])
    .sort((a, b) => (a[1] < b[1] ? -1 : 1))
    .slice(0, supermajority)
    .map((a) => a[0])
    .sort((a, b) => ethers.toNumber(a) - ethers.toNumber(b));
  for (let index of sample) {
    const returned = await relayer
      .connect(validators[index])
      .query(caller, callee, call);
    expect(success).to.satisfy((v) => !v || v == returned[0]);
    var success = returned[0];
    expect(result).to.satisfy((v) => !v || v == returned[1]);
    var result = returned[1];
  }
  return { success, result };
}

export async function dispatchCall(
  validators,
  relayer,
  caller,
  callee,
  call,
  success,
  callback,
  nonce,
  signerIndices,
  signatures
) {
  // the next leader dispatches the relayed call
  const leaderIndex =
    signerIndices[Math.floor(Math.random() * signerIndices.length)];
  const tx = await relayer
    .connect(validators[leaderIndex])
    .dispatch(caller, callee, call, callback, nonce, signerIndices, signatures);
  const rcpt = await tx.wait();
  await expect(tx).to.emit(relayer, "Dispatched").withArgs(
    caller,
    callback,
    success, // expected boolean outcome or anyValue if outcome is not known in advance
    anyValue,
    nonce
  );

  // other vaidators see the Dispatched event and retrieve the result of the relayed call
  var result;
  for (const index of signerIndices) {
    const blockNum = await ethers.provider.getBlockNumber();
    const logs = await validators[index].provider.getLogs({
      fromBlock: blockNum - 100,
      toBlock: blockNum,
      address: await relayer.getAddress(),
      topics: [
        ethers.id("Dispatched(address,bytes4,bool,bytes,uint256)"),
        ethers.zeroPadValue(caller, 32),
        ethers.toBeHex(nonce, 32),
      ],
    });
    const res = ethers.AbiCoder.defaultAbiCoder().decode(
      ["bytes4", "bool", "bytes"],
      logs[0].data
    );
    expect(res[0]).to.equal(callback);
    if (success != anyValue) expect(res[1]).to.equal(success);
    else success = res[1];
    result = res[2];
  }
  return { success, result };
}

export async function confirmResult(
  validators,
  relayer,
  caller,
  callback,
  success,
  result,
  nonce
) {
  // validators sign the hash of the Dispatched event data and submit their signature
  const message = ethers.AbiCoder.defaultAbiCoder().encode(
    ["address", "bytes4", "bool", "bytes", "uint256"],
    [caller, callback, success, result, nonce]
  );
  const hash = ethers.hashMessage(ethers.getBytes(message));
  const signatures = [];
  const supermajority = Math.floor((validators.length * 2) / 3) + 1;
  const signerIndices = [...Array(validators.length).keys()]
    .map((a) => [a, Math.random()])
    .sort((a, b) => (a[1] < b[1] ? -1 : 1))
    .slice(0, supermajority)
    .map((a) => a[0])
    .sort((a, b) => ethers.toNumber(a) - ethers.toNumber(b));
  for (const index of signerIndices) {
    signatures.push(
      await validators[index].signMessage(ethers.getBytes(message))
    );
    const tx = await relayer
      .connect(validators[index])
      .echo(hash, index, signatures.slice(-1)[0]);
    const rcpt = await tx.wait();
    await expect(tx)
      .to.emit(relayer, "Echoed")
      .withArgs(hash, index, signatures.slice(-1)[0]);
  }

  // validators retrieve the signatures submitted by other validators from the Echoed events
  for (const index of signerIndices) {
    const blockNum = await ethers.provider.getBlockNumber();
    const logs = await validators[index].provider.getLogs({
      fromBlock: blockNum - 100,
      toBlock: blockNum,
      address: await relayer.getAddress(),
      topics: [ethers.id("Echoed(bytes32,uint16,bytes)"), hash],
    });
    for (let i = 0; i < logs.length; i++) {
      const res = ethers.AbiCoder.defaultAbiCoder().decode(
        ["uint16", "bytes"],
        logs[i].data
      );
      expect(res[0]).to.equal(signerIndices[i]);
      expect(res[1]).to.equal(signatures[i]);
    }
  }

  return { signerIndices, signatures };
}

export async function deliverResult(
  validators,
  relayer,
  caller,
  callback,
  success,
  result,
  nonce,
  signerIndices,
  signatures
) {
  // the next leader delivers the result to the caller contract and resumes its execution
  const leaderIndex =
    signerIndices[Math.floor(Math.random() * signerIndices.length)];
  const tx = await relayer
    .connect(validators[leaderIndex])
    .resume(
      caller,
      callback,
      success,
      result,
      nonce,
      signerIndices,
      signatures
    );
  const rcpt = await tx.wait();
  await expect(tx)
    .to.emit(relayer, "Resumed")
    .withArgs(
      caller,
      ethers.concat([
        callback,
        ethers.AbiCoder.defaultAbiCoder().encode(
          ["bool", "bytes", "uint256"],
          [success, result, nonce]
        ),
      ]),
      true,
      "0x",
      nonce
    );

  // other validators see the Resumed event and do not attmpt to deliver the result again
  for (const index of signerIndices) {
    const blockNum = await ethers.provider.getBlockNumber();
    const logs = await validators[index].provider.getLogs({
      fromBlock: blockNum - 100,
      toBlock: blockNum,
      address: await relayer.getAddress(),
      topics: [
        ethers.id("Resumed(address,bytes,bool,bytes,uint256)"),
        ethers.zeroPadValue(caller, 32),
        ethers.toBeHex(nonce, 32),
      ],
    });
    const res = ethers.AbiCoder.defaultAbiCoder().decode(
      ["bytes", "bool", "bytes"],
      logs[0].data
    );
    expect(res[1]).to.equal(true);
    expect(res[2]).to.equal("0x");
  }
}
