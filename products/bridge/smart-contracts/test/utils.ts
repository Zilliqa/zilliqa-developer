// TODO: fix utils

/*
import { expect } from "chai";
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { createProvider } from "hardhat/internal/core/providers/construction";
import { HardhatEthersProvider } from "@nomicfoundation/hardhat-ethers/internal/hardhat-ethers-provider";
import hre from "hardhat";
import { ethers } from "hardhat";
import {
  AddressLike,
  BytesLike,
  ContractTransactionReceipt,
  Signer,
} from "ethers";
import { Relayer, Collector } from "../typechain-types";

export async function dispatchMessage(
  sourceNetwork: number,
  targetNetwork: number,
  sourceChainId: bigint,
  targetChainId: bigint,
  validatorsSource: Signer[],
  relayerSource: Relayer,
  validatorsTarget: Signer[],
  relayerTarget: Relayer,
  collectorValidators: Signer[],
  collector: Collector,
  isSuccess: boolean,
  isQuery: boolean,
  expectedResponse?: BytesLike
) {
  switchNetwork(sourceNetwork);

  const { caller, callee, call, readonly, callback, nonce } = (
    await obtainCalls(validatorsSource, relayerSource)
  ).slice(-1)[0];

  expect(readonly).to.equal(isQuery);

  const callSignatures = await confirmCall(
    collectorValidators,
    collector,
    sourceChainId,
    targetChainId,
    caller,
    callee,
    call,
    readonly,
    callback,
    nonce
  );

  switchNetwork(targetNetwork);

  let success, result, dispatchTxn;
  if (isQuery) {
    const query = await queryCall(
      validatorsTarget,
      relayerTarget,
      caller,
      callee,
      call
    );
    success = query.success;
    result = query.response;
  } else {
    const txResponse = await dispatchCall(
      validatorsTarget,
      sourceChainId,
      targetChainId,
      relayerTarget,
      caller,
      callee,
      call,
      readonly,
      callback,
      nonce,
      callSignatures
    );
    dispatchTxn = await txResponse.wait();
    // Calculate gas usage of dispatch
    // console.log("gas used: ", dispatchTxn?.gasUsed);
    // const x = await relayerTarget.gasRefund(
    //   await validatorsTarget[0].getAddress()
    // );
    // console.log(x / (dispatchTxn?.gasPrice ?? 1n));
    // console.log("gas price: ", dispatchTxn?.gasPrice);

    if (!dispatchTxn) {
      expect.fail("tx is null");
    }
    const dispatch = await verifyDispatchCall(
      validatorsTarget,
      sourceChainId,
      relayerTarget,
      caller,
      isSuccess,
      callback,
      nonce,
      dispatchTxn
    );
    success = dispatch.success;
    result = dispatch.result;
  }

  expect(success).to.equal(isSuccess);
  if (expectedResponse) {
    expect(result).to.equal(expectedResponse);
  }

  switchNetwork(sourceNetwork);

  const resultSignatures = await confirmResult(
    collectorValidators,
    collector,
    sourceChainId,
    targetChainId,
    caller,
    callback,
    success,
    result,
    nonce
  );

  const txResponse = await deliverResult(
    validatorsSource,
    relayerSource,
    sourceChainId,
    targetChainId,
    caller,
    callback,
    success,
    result,
    nonce,
    resultSignatures
  );
  const deliveryTxn = await txResponse.wait();
  if (!deliveryTxn) {
    expect.fail("tx is null");
  }
  await verifyDeliveryResult(
    validatorsSource,
    relayerSource,
    targetChainId,
    caller,
    callback,
    success,
    result,
    nonce,
    deliveryTxn
  );
  return {
    nonce,
    callSignatures,
    resultSignatures,
    result,
    deliveryTxn,
    dispatchTxn,
  };
}

export async function setupBridge() {
  const validatorSize = 15;

  switchNetwork(1);

  const signers1 = await ethers.getSigners();
  const twinDeployer1 = signers1[signers1.length - 1];
  const tester1 = signers1[signers1.length - 2];
  const validators1 = signers1.slice(1, validatorSize + 1);
  const validatorAddresses = await Promise.all(
    validators1.map(async (s) => s.getAddress())
  );

  const validatorManager1 = await ethers
    .deployContract("ValidatorManager", [validatorAddresses], twinDeployer1)
    .then(async (c) => c.waitForDeployment());
  expect(await validatorManager1.getValidators()).to.have.lengthOf(
    validatorSize
  );

  const relayer1 = await ethers
    .deployContract(
      "Relayer",
      [await validatorManager1.getAddress()],
      twinDeployer1
    )
    .then((x) => x.waitForDeployment());
  await relayer1.waitForDeployment();

  const collector = await ethers
    .deployContract("Collector", [await validatorManager1.getAddress()])
    .then(async (c) => c.waitForDeployment());

  const network1 = await ethers.provider.getNetwork();
  const chainId1 = network1.chainId;

  switchNetwork(2);

  const signers2 = await ethers.getSigners();
  const twinDeployer2 = signers2[signers2.length - 1];
  const tester2 = signers2[signers2.length - 2];
  const validators2 = signers2.slice(1, validatorSize + 1);

  const validatorManager2 = await ethers
    .deployContract("ValidatorManager", [validatorAddresses], twinDeployer2)
    .then(async (c) => c.waitForDeployment());
  expect(await validatorManager2.getValidators()).to.have.lengthOf(
    validatorSize
  );

  const relayer2 = await ethers
    .deployContract("Relayer", [validatorManager2], twinDeployer2)
    .then((x) => x.waitForDeployment());

  const network2 = await ethers.provider.getNetwork();
  const chainId2 = network2.chainId;

  switchNetwork(1);

  return {
    collector,
    relayer1,
    relayer2,
    validators1,
    validators2,
    tester1,
    tester2,
    twinDeployer1,
    twinDeployer2,
    chainId1,
    chainId2,
  };
}

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
  const filter = relayer.filters.Relayed;
  // Select random validator to query (in tests they use the same provider, but not in reality)
  const logs = await relayer
    .connect(validators[randIndex])
    .queryFilter(filter, "earliest", "finalized");
  return logs.map(
    ({
      args: [targetChainId, caller, callee, call, readonly, callback, nonce],
    }) => ({
      targetChainId,
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

function getSampleOfValidators(
  validators: Signer[],
  supermajority: number
): Signer[] {
  return getRandomSample<Signer>(validators, supermajority);
}

function orderSignaturesBySignerAddress(hash: string, signatures: string[]) {
  return signatures.sort((a, b) => {
    const signerA = ethers.toBigInt(ethers.recoverAddress(hash, a));
    const signerB = ethers.toBigInt(ethers.recoverAddress(hash, b));
    return signerA < signerB ? -1 : 1;
  });
}

export async function confirmCall(
  validators: Signer[],
  collector: Collector,
  sourceChainId: bigint,
  targetChainId: bigint,
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
    [
      "uint256",
      "uint256",
      "address",
      "address",
      "bytes",
      "bool",
      "bytes4",
      "uint256",
    ],
    [
      sourceChainId,
      targetChainId,
      caller,
      callee,
      call,
      readonly,
      callback,
      nonce,
    ]
  );
  const hash = ethers.hashMessage(ethers.getBytes(message));
  const supermajority = Math.floor((validators.length * 2) / 3) + 1;
  const sampledValidators = getSampleOfValidators(validators, supermajority);

  // Collect and emit the signatures
  const signatures: string[] = [];
  for (const validator of sampledValidators) {
    signatures.push(await validator.signMessage(ethers.getBytes(message)));
    const tx = await collector
      .connect(validator)
      .echo(hash, signatures.slice(-1)[0]);
    await tx.wait();
    await expect(tx)
      .to.emit(collector, "Echoed")
      .withArgs(hash, signatures.slice(-1)[0]);
  }
  const validatorAddresses = await Promise.all(
    validators.map(async (v) => v.getAddress())
  );
  // validators retrieve the signatures submitted by other validators from the Echoed events
  for (const validator of validators) {
    const filter = collector.filters.Echoed(hash);
    const logs = await collector.connect(validator).queryFilter(filter);

    logs.forEach((log, i) => {
      const [hash, signature] = log.args;
      const signer = ethers.recoverAddress(hash, signature);

      // TODO: make sure to also check that the signature is valid and unique
      expect(signer).to.be.oneOf(validatorAddresses);
      expect(signature).to.equal(signatures[i]);
    });
  }
  return signatures;
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

async function verifyDispatchCall(
  validators: Signer[],
  sourceChainId: bigint,
  relayer: Relayer,
  caller: AddressLike,
  success: boolean,
  callback: BytesLike,
  nonce: bigint,
  tx: ContractTransactionReceipt
) {
  await expect(tx).to.emit(relayer, "Dispatched").withArgs(
    sourceChainId,
    caller,
    callback,
    success, // expected boolean outcome or anyValue if outcome is not known in advance
    anyValue,
    nonce
  );

  // other validators see the Dispatched event and retrieve the result of the relayed call
  const results = await Promise.all(
    validators.map(async (validator) => {
      const filter = relayer.filters.Dispatched(
        undefined,
        caller,
        undefined,
        undefined,
        undefined,
        nonce
      );
      const logs = await relayer
        .connect(validator)
        .queryFilter(filter, "earliest", "finalized");

      return logs.slice(-1)[0].args;
    })
  );
  // check that the result is the same for all validators
  for (const result of results) {
    expect(result).is.deep.equal(results[0]);
  }

  return {
    success: results[0].success,
    result: results[0].response,
  };
}

export async function dispatchCall(
  validators: Signer[],
  sourceChainId: bigint,
  targetChainId: bigint,
  relayer: Relayer,
  caller: AddressLike,
  callee: AddressLike,
  call: BytesLike,
  readonly: boolean,
  callback: BytesLike,
  nonce: bigint,
  signatures: string[]
) {
  // the next leader dispatches the relayed call
  const leaderValidator = validators[0];
  const message = ethers.AbiCoder.defaultAbiCoder().encode(
    [
      "uint256",
      "uint256",
      "address",
      "address",
      "bytes",
      "bool",
      "bytes4",
      "uint256",
    ],
    [
      sourceChainId,
      targetChainId,
      caller,
      callee,
      call,
      readonly,
      callback,
      nonce,
    ]
  );
  const hash = ethers.hashMessage(ethers.getBytes(message));
  const orderedSignatures = orderSignaturesBySignerAddress(hash, signatures);

  return relayer
    .connect(leaderValidator)
    .dispatch(
      sourceChainId,
      caller,
      callee,
      call,
      callback,
      nonce,
      orderedSignatures
    );
}

export async function confirmResult(
  validators: Signer[],
  collector: Collector,
  sourceChainId: bigint,
  targetChainId: bigint,
  caller: AddressLike,
  callback: BytesLike,
  success: boolean,
  result: BytesLike,
  nonce: bigint
) {
  // validators sign the hash of the Dispatched event data and submit their signature
  const message = ethers.AbiCoder.defaultAbiCoder().encode(
    ["uint256", "uint256", "address", "bytes4", "bool", "bytes", "uint256"],
    [sourceChainId, targetChainId, caller, callback, success, result, nonce]
  );
  const hash = ethers.hashMessage(ethers.getBytes(message));
  const supermajority = Math.floor((validators.length * 2) / 3) + 1;
  const sampledValidators = getSampleOfValidators(validators, supermajority);

  const signatures: string[] = [];
  for (const validator of sampledValidators) {
    const signature = await validator.signMessage(ethers.getBytes(message));
    signatures.push(signature);

    const tx = await collector.connect(validator).echo(hash, signature);
    await tx.wait();
    await expect(tx).to.emit(collector, "Echoed").withArgs(hash, signature);
  }

  // validators retrieve the signatures submitted by other validators from the Echoed events
  for (const validator of validators) {
    const filter = collector.filters.Echoed(hash);
    const logs = await collector.connect(validator).queryFilter(filter);

    expect(logs.map((log) => log.args.signature)).to.have.members(signatures);
  }

  return signatures;
}

export async function deliverResult(
  validators: Signer[],
  relayer: Relayer,
  sourceChainId: bigint,
  targetChainId: bigint,
  caller: AddressLike,
  callback: BytesLike,
  success: boolean,
  result: BytesLike,
  nonce: bigint,
  signatures: string[]
) {
  // the next leader delivers the result to the caller contract and resumes its execution
  const leaderValidator =
    validators[Math.floor(Math.random() * validators.length)];
  const message = ethers.AbiCoder.defaultAbiCoder().encode(
    ["uint256", "uint256", "address", "bytes4", "bool", "bytes", "uint256"],
    [sourceChainId, targetChainId, caller, callback, success, result, nonce]
  );
  const hash = ethers.hashMessage(ethers.getBytes(message));
  const orderedSignatures = orderSignaturesBySignerAddress(hash, signatures);
  return relayer
    .connect(leaderValidator)
    .resume(
      targetChainId,
      caller,
      callback,
      success,
      result,
      nonce,
      orderedSignatures
    );
}

async function verifyDeliveryResult(
  validators: Signer[],
  relayer: Relayer,
  targetChainId: bigint,
  caller: AddressLike,
  callback: BytesLike,
  success: boolean,
  result: BytesLike,
  nonce: bigint,
  tx: ContractTransactionReceipt
) {
  await expect(tx)
    .to.emit(relayer, "Resumed")
    .withArgs(
      targetChainId,
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
  for (const validator of validators) {
    const filter = relayer.filters.Resumed(
      targetChainId,
      caller,
      undefined,
      undefined,
      undefined,
      nonce
    );
    const logs = await relayer
      .connect(validator)
      .queryFilter(filter, "earliest", "finalized");

    expect(logs[0].args.success).to.equal(true);
    expect(logs[0].args.response).to.equal("0x");
  }
}

*/
