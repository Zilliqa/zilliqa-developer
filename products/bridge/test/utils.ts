import { expect } from "chai";
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { createProvider } from "hardhat/internal/core/providers/construction";
import { HardhatEthersProvider } from "@nomicfoundation/hardhat-ethers/internal/hardhat-ethers-provider";
import hre from "hardhat";
import { ethers } from "hardhat";
import { AddressLike, BytesLike, Signer } from "ethers";
import { CollectorRelayer, Relayer } from "../typechain-types";

export async function switchNetwork(id = 0) {
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
    hre.network.name = "hardhat";
    hre.network.config = hre.config.networks["hardhat"];
    hre.network.provider = await createProvider(
      hre.config,
      hre.network.name,
      hre.artifacts
    );
    hre.ethers.provider = new HardhatEthersProvider(
      hre.network.provider,
      hre.network.name
    );
  }
}

export async function obtainCalls(validators: Signer[], relayer: Relayer) {
  // validators see the Relayed event
  const randIndex = Math.floor(Math.random() * validators.length);
  const blockNum = await ethers.provider.getBlockNumber();
  const filter = relayer.filters.Relayed;
  // Select random validator to query (in tests they use the same provider, but not in reality)
  const logs = await relayer
    .connect(validators[randIndex])
    .queryFilter(filter, blockNum - 100, blockNum);
  return logs.map(
    ({ args: [caller, callee, call, readonly, callback, nonce] }) => ({
      caller,
      callee,
      call,
      readonly,
      callback,
      nonce,
    })
  );
}

function getRandomSample<T>(items: T[], n: number): T[] {
  return items
    .map((a) => [a, Math.random()] as [T, number])
    .sort((a, b) => (a[1] < b[1] ? -1 : 1))
    .slice(0, n)
    .map((a) => a[0]);
}

function getSampleOfValidatorsIndices(
  validatorsCount: number,
  supermajority: number
): number[] {
  return getRandomSample(
    [...Array(validatorsCount).keys()],
    supermajority
  ).sort((a, b) => ethers.toNumber(a) - ethers.toNumber(b));
}

export async function confirmCall(
  validators: Signer[],
  relayer: CollectorRelayer,
  caller: AddressLike,
  callee: AddressLike,
  call: BytesLike,
  readonly: boolean,
  callback: BytesLike,
  nonce: bigint
) {
  // validators sign the hash of the Relayed event data and submit their signature
  // Prepare the hashed message
  const message = ethers.AbiCoder.defaultAbiCoder().encode(
    ["address", "address", "bytes", "bool", "bytes4", "uint256"],
    [caller, callee, call, readonly, callback, nonce]
  );
  const hash = ethers.hashMessage(ethers.getBytes(message));
  const supermajority = Math.floor((validators.length * 2) / 3) + 1;
  const signerIndices = getSampleOfValidatorsIndices(
    validators.length,
    supermajority
  );

  // Collect and emit the signatures
  const signatures: string[] = [];
  for (const index of signerIndices) {
    signatures.push(
      await validators[index].signMessage(ethers.getBytes(message))
    );
    const tx = await relayer
      .connect(validators[index])
      .echo(hash, index, signatures.slice(-1)[0]);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer, "Echoed")
      .withArgs(hash, index, signatures.slice(-1)[0]);
  }

  // validators retrieve the signatures submitted by other validators from the Echoed events
  for (const index of signerIndices) {
    const blockNum = await ethers.provider.getBlockNumber();
    const filter = relayer.filters.Echoed(hash);
    const logs = await relayer
      .connect(validators[index])
      .queryFilter(filter, blockNum - 100, blockNum);

    logs.forEach((log, i) => {
      const [, index, signature] = log.args;

      expect(Number(index)).to.be.oneOf(signerIndices);
      expect(signature).to.equal(signatures[i]);
    });
  }
  return { signerIndices, signatures };
}

export async function queryCall(
  validators: Signer[],
  relayer: Relayer,
  caller: AddressLike,
  callee: AddressLike,
  call: BytesLike
) {
  // the supermajority of the validators retrieves the result using a view call
  const supermajority = Math.floor((validators.length * 2) / 3) + 1;
  const sample = [...Array(validators.length).keys()]
    .map((a) => [a, Math.random()])
    .sort((a, b) => (a[1] < b[1] ? -1 : 1))
    .slice(0, supermajority)
    .map((a) => a[0])
    .sort((a, b) => ethers.toNumber(a) - ethers.toNumber(b));

  // Make sure all validators get the same result
  const results = await Promise.all(
    sample.map(async (index) =>
      relayer.connect(validators[index]).query(caller, callee, call)
    )
  );
  const [success, response] = results[0];
  expect(results.every(([s, r]) => s === success && r === response)).is.true;
  return { success, response };
}

export async function dispatchCall(
  validators: Signer[],
  relayer: Relayer,
  caller: AddressLike,
  callee: AddressLike,
  call: BytesLike,
  success: boolean,
  callback: BytesLike,
  nonce: bigint,
  signerIndices: number[],
  signatures: string[]
) {
  // the next leader dispatches the relayed call
  const leaderIndex =
    signerIndices[Math.floor(Math.random() * signerIndices.length)];
  const tx = await relayer
    .connect(validators[leaderIndex])
    .dispatch(caller, callee, call, callback, nonce, signerIndices, signatures);
  await tx.wait();
  expect(tx).to.emit(relayer, "Dispatched").withArgs(
    caller,
    callback,
    success, // expected boolean outcome or anyValue if outcome is not known in advance
    anyValue,
    nonce
  );

  // other vaidators see the Dispatched event and retrieve the result of the relayed call
  const results = await Promise.all(
    signerIndices.map(async (index) => {
      const blockNum = await ethers.provider.getBlockNumber();
      const filter = relayer.filters.Dispatched(
        caller,
        undefined,
        undefined,
        undefined,
        nonce
      );
      const logs = await relayer
        .connect(validators[index])
        .queryFilter(filter, blockNum - 100, blockNum);

      return logs[0].args;
    })
  );
  // check that the result is the same for all validators
  for (const result of results) {
    expect(result).is.deep.equal(results[0]);
  }

  return { success: results[0].success, result: results[0].response };
}

export async function confirmResult(
  validators: Signer[],
  relayer: CollectorRelayer,
  caller: AddressLike,
  callback: BytesLike,
  success: boolean,
  result: BytesLike,
  nonce: bigint
) {
  // validators sign the hash of the Dispatched event data and submit their signature
  const message = ethers.AbiCoder.defaultAbiCoder().encode(
    ["address", "bytes4", "bool", "bytes", "uint256"],
    [caller, callback, success, result, nonce]
  );
  const hash = ethers.hashMessage(ethers.getBytes(message));
  const supermajority = Math.floor((validators.length * 2) / 3) + 1;
  const signerIndices = getSampleOfValidatorsIndices(
    validators.length,
    supermajority
  );

  const signatures: string[] = [];
  for (const index of signerIndices) {
    const signature = await validators[index].signMessage(
      ethers.getBytes(message)
    );
    signatures.push(signature);

    const tx = await relayer
      .connect(validators[index])
      .echo(hash, index, signature);
    await tx.wait();
    expect(tx).to.emit(relayer, "Echoed").withArgs(hash, index, signature);
  }

  // validators retrieve the signatures submitted by other validators from the Echoed events
  for (const index of signerIndices) {
    const blockNum = await ethers.provider.getBlockNumber();

    const filter = relayer.filters.Echoed(hash);
    const logs = await relayer
      .connect(validators[index])
      .queryFilter(filter, blockNum - 100, blockNum);

    expect(logs.map((log) => Number(log.args.index))).to.have.members(
      signerIndices
    );
    expect(logs.map((log) => log.args.signature)).to.have.members(signatures);
  }

  return { signerIndices, signatures };
}

export async function deliverResult(
  validators: Signer[],
  relayer: Relayer,
  caller: AddressLike,
  callback: BytesLike,
  success: boolean,
  result: BytesLike,
  nonce: bigint,
  signerIndices: number[],
  signatures: string[]
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
  await tx.wait();
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

  // other validators see the Resumed event and do not attempt to deliver the result again
  for (const index of signerIndices) {
    const blockNum = await ethers.provider.getBlockNumber();

    const filter = relayer.filters.Resumed(
      caller,
      undefined,
      undefined,
      undefined,
      nonce
    );
    const logs = await relayer
      .connect(validators[index])
      .queryFilter(filter, blockNum - 100, blockNum);

    expect(logs[0].args.success).to.equal(true);
    expect(logs[0].args[3]).to.equal("0x");
  }
}
