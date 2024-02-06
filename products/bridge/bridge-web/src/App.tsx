import { ConnectButton } from "@rainbow-me/rainbowkit";
import zilliqa from "./assets/zilliqa.png";
import { fromBech32Address, toBech32Address } from "@zilliqa-js/crypto";
import { validation } from "@zilliqa-js/util";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faArrowRight,
  faArrowUpRightFromSquare,
  faChevronDown,
  faRepeat,
} from "@fortawesome/free-solid-svg-icons";
import { useEffect, useState } from "react";
import { Chains, TokenConfig, chainConfigs } from "./config/config";
import {
  erc20ABI,
  useAccount,
  useContractRead,
  useContractWrite,
  useNetwork,
  usePrepareContractWrite,
  useSwitchNetwork,
  useWaitForTransaction,
} from "wagmi";
import { formatUnits, parseUnits } from "viem";
import { Id, toast } from "react-toastify";
import { TokenManagerAbi } from "./abi/TokenManager";

type TxnType = "approve" | "bridge";

function App() {
  const [fromChain, setFromChain] = useState<Chains>(
    Object.values(chainConfigs)[0].chain
  );
  const { address: account } = useAccount();
  const [toChain, setToChain] = useState<Chains>(
    Object.values(chainConfigs)[1].chain
  );
  const [amount, setAmount] = useState<string>("");
  const isAmountNonZero = Number(amount) > 0;
  const fromChainConfig = chainConfigs[fromChain]!;
  const toChainConfig = chainConfigs[toChain]!;
  const { switchNetwork } = useSwitchNetwork();
  const { chain } = useNetwork();
  const [latestTxn, setLatestTxn] = useState<[TxnType, `0x${string}`]>();
  const [loadingId, setLoadingId] = useState<Id>();

  const [recipient, setRecipient] = useState<string>();
  const [token, selectedToken] = useState<TokenConfig>(
    Object.values(chainConfigs)[0].tokens[0]
  );

  useEffect(() => {
    switchNetwork && switchNetwork(fromChainConfig.chainId);
  }, [fromChainConfig, switchNetwork]);

  useEffect(() => {
    selectedToken(fromChainConfig.tokens[0]);
  }, [fromChain, fromChainConfig.tokens]);

  useEffect(() => {
    setRecipient(account);
  }, [account]);

  useEffect(() => {
    if (chain !== fromChainConfig.wagmiChain) {
      const newFromChain = Object.values(chainConfigs).find(
        (chainConfig) => chainConfig.chainId == chain?.id
      );
      if (!newFromChain?.chain) {
        return;
      }
      if (newFromChain === toChainConfig) {
        setToChain(fromChain);
      }
      setFromChain(newFromChain?.chain);
    }
  }, [chain, fromChain, fromChainConfig.wagmiChain, toChainConfig]);

  const { data: decimals } = useContractRead({
    abi: erc20ABI,
    functionName: "decimals",
    address: token.address,
    enabled: !!token.address,
  });
  const { data: balance } = useContractRead({
    abi: erc20ABI,
    functionName: "balanceOf",
    args: account ? [account!] : undefined,
    address: token.address,
    enabled: !!account && !!token.address,
    watch: true,
  });

  const { data: allowance } = useContractRead({
    abi: erc20ABI,
    functionName: "allowance",
    address: token.address,
    args: [account!, fromChainConfig.tokenManagerAddress],
    enabled:
      !!account && !!token.address && !!fromChainConfig.tokenManagerAddress,
    watch: true,
  });

  const hasEnoughAllowance =
    !!decimals && isAmountNonZero
      ? (allowance ?? 0n) >= parseUnits(amount!, decimals)
      : true;
  const hasEnoughBalance =
    decimals && balance ? parseUnits(amount!, decimals) <= balance : false;
  const validBech32Address = recipient && validation.isBech32(recipient);
  const validEthAddress = recipient && validation.isAddress(recipient);
  const hasValidAddress = recipient
    ? validBech32Address != validEthAddress
    : true;
  const ethRecipient = validBech32Address
    ? fromBech32Address(recipient)
    : recipient;

  const { config: transferConfig } = usePrepareContractWrite({
    address: fromChainConfig.tokenManagerAddress,
    abi: TokenManagerAbi,
    args: [
      token.address,
      BigInt(toChainConfig.chainId),
      ethRecipient as `0x${string}`,
      parseUnits(amount!, decimals ?? 0),
    ],
    functionName: "transfer",
    enabled: !!(
      toChainConfig &&
      fromChainConfig &&
      !fromChainConfig.isZilliqa &&
      ethRecipient &&
      amount &&
      decimals
    ),
  });

  const { writeAsync: bridge, isLoading: isLoadingBridge } =
    useContractWrite(transferConfig);

  // From Zilliqa Bridging
  const {
    writeAsync: bridgeFromZilliqa,
    isLoading: isLoadingBridgeFromZilliqa,
  } = useContractWrite({
    mode: "prepared",
    request: {
      address: fromChainConfig.tokenManagerAddress,
      chain: fromChainConfig.wagmiChain,
      account: account!,
      abi: TokenManagerAbi,
      args: [
        token.address,
        BigInt(toChainConfig.chainId),
        ethRecipient as `0x${string}`,
        parseUnits(amount!, decimals ?? 0),
      ],
      functionName: "transfer",
      gas: 600_000n,
      type: "legacy",
    },
  });

  // Approvals
  const { config: approveConfig } = usePrepareContractWrite({
    address: token.address,
    abi: erc20ABI,
    args: [
      fromChainConfig.tokenManagerAddress,
      parseUnits(amount!, decimals ?? 0),
    ],
    functionName: "approve",
    gas: fromChainConfig.isZilliqa ? 400_000n : undefined,
    type: fromChainConfig.isZilliqa ? "legacy" : "eip1559",
  });

  const { writeAsync: approve, isLoading: isLoadingApprove } =
    useContractWrite(approveConfig);

  const canBridge =
    amount &&
    hasValidAddress &&
    hasEnoughAllowance &&
    hasEnoughBalance &&
    (fromChainConfig.isZilliqa
      ? !isLoadingBridgeFromZilliqa
      : !isLoadingBridge);

  const {
    data: txnReceipt,
    isLoading: isWaitingForTxn,
    error,
    refetch,
  } = useWaitForTransaction({
    hash: latestTxn?.[1],
    enabled: !!latestTxn?.[1],
  });

  useEffect(() => {
    if (error) {
      // Little hack to get Zilliqa to refetch the pending txns
      refetch();
    }
  }, [error, refetch]);

  useEffect(() => {
    if (txnReceipt && loadingId && latestTxn) {
      let description;
      if (latestTxn[0] === "bridge") {
        description = (
          <div>
            Bridge request txn sent. From {fromChainConfig.name} to{" "}
            {toChainConfig.name} {amount} {token.name} tokens. View on{" "}
            <a
              className="link text-ellipsis w-10"
              onClick={() =>
                window.open(
                  `${fromChainConfig.blockExplorer}${txnReceipt.transactionHash}`,
                  "_blank"
                )
              }
            >
              block explorer
            </a>
          </div>
        );
        setAmount("");
      } else if (latestTxn[0] === "approve") {
        description = (
          <div>
            Approve txn successful. View on{" "}
            <a
              className="link text-ellipsis w-10"
              onClick={() =>
                window.open(
                  `${fromChainConfig.blockExplorer}${txnReceipt.transactionHash}`,
                  "_blank"
                )
              }
            >
              block explorer
            </a>
          </div>
        );
      } else {
        return;
      }
      toast.update(loadingId, {
        render: description,
        type: "success",
        isLoading: false,
      });
      setLoadingId(undefined);
      setLatestTxn(undefined);
    }
  }, [
    isWaitingForTxn,
    txnReceipt,
    loadingId,
    latestTxn,
    fromChainConfig.name,
    fromChainConfig.blockExplorer,
    toChainConfig.name,
    amount,
    token.name,
  ]);

  useEffect(() => {
    if (!loadingId && isWaitingForTxn && latestTxn) {
      const id = toast.loading("Transaction being processed...");
      setLoadingId(id);
    }
  }, [isWaitingForTxn, latestTxn, loadingId]);

  const showLoadingButton =
    isLoadingBridgeFromZilliqa ||
    isLoadingBridge ||
    isLoadingApprove ||
    isWaitingForTxn;

  return (
    <>
      <div className="h-screen flex items-center justify-center">
        <div className="fixed top-0 navbar py-6 px-10 ">
          <div className="flex-1 hidden sm:block">
            <img src={zilliqa} className="h-16" alt="Zilliqa Logo" />
          </div>
          <div className="flex-none">
            <ConnectButton />
          </div>
        </div>

        <div className="card min-h-96 bg-neutral shadow-xl">
          <div className="card-body">
            <div className="card-title">
              <p className="text-4xl text-center tracking-wide">BRIDGE</p>
            </div>

            <div className="form-control">
              <div className="label">
                <span>Networks</span>
              </div>
              <div className="join">
                <div className="dropdown w-1/2">
                  <div tabIndex={0} role="button" className="btn w-full">
                    <p className="w-12">{fromChainConfig.name}</p>
                    <FontAwesomeIcon
                      icon={faChevronDown}
                      className="ml-auto"
                      color="white"
                    />
                  </div>
                  <ul
                    tabIndex={0}
                    className="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-full"
                  >
                    {Object.values(chainConfigs)
                      .filter(
                        (config) => config.chain !== fromChainConfig.chain
                      )
                      .map(({ chain, name }) => (
                        <li
                          key={`from${chain}`}
                          onClick={() => {
                            if (chain === toChain) {
                              setToChain(fromChain);
                            }
                            setFromChain(chain);
                          }}
                        >
                          <a>{name}</a>
                        </li>
                      ))}
                  </ul>
                </div>
                <FontAwesomeIcon
                  className="w-1/6 self-center"
                  icon={faArrowRight}
                  color="white"
                />
                <div className="dropdown w-1/2">
                  <div tabIndex={0} role="button" className="btn w-full">
                    <p className="w-12">{toChainConfig.name}</p>
                    <FontAwesomeIcon
                      icon={faChevronDown}
                      color="white"
                      className="ml-auto"
                    />
                  </div>
                  <ul
                    tabIndex={0}
                    className="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-full"
                  >
                    {Object.values(chainConfigs)
                      .filter(({ chain }) => chain !== toChainConfig.chain)
                      .map(({ chain, name }) => (
                        <li
                          key={`to${chain}`}
                          onClick={() => {
                            if (chain === fromChain) {
                              setFromChain(toChain);
                            }
                            setToChain(chain);
                            blur();
                          }}
                        >
                          <a>{name}</a>
                        </li>
                      ))}
                  </ul>
                </div>
              </div>
            </div>

            <div className="form-control">
              <div className="label">
                <span>Recipient</span>
              </div>
              <div className="join">
                <div className="indicator">
                  <button
                    className="btn join-item"
                    disabled={!hasValidAddress}
                    onClick={() => {
                      setRecipient((_recipient) => {
                        if (!_recipient) {
                          return _recipient;
                        }
                        if (validation.isBech32(_recipient)) {
                          return fromBech32Address(_recipient!);
                        }
                        if (validation.isAddress(_recipient)) {
                          return toBech32Address(_recipient);
                        }
                      });
                    }}
                  >
                    <FontAwesomeIcon
                      icon={faRepeat}
                      color="white"
                      className="ml-auto"
                    />
                  </button>
                </div>
                <input
                  className={`input join-item input-bordered w-full font-mono text-sm text-end ${
                    !hasValidAddress && "input-warning"
                  }`}
                  placeholder="Address"
                  value={recipient}
                  onChange={({ target }) => setRecipient(target.value)}
                />
              </div>
              {!hasValidAddress && (
                <div className="label align-bottom place-content-end">
                  <span className="label-text-alt text-warning">
                    Invalid Address
                  </span>
                </div>
              )}
            </div>

            <div className="form-control">
              <div className="label">
                <span>Token</span>
                <span className="label-text-alt self-end">
                  Balance:{" "}
                  {balance !== undefined && decimals
                    ? formatUnits(balance, decimals)
                    : null}
                </span>
              </div>
              <div className="join">
                <div className="indicator">
                  <button
                    onClick={() => window.open(token.blockExplorer, "_blank")}
                    className="btn join-item w-32"
                  >
                    {token.logo && (
                      <img
                        src={token.logo}
                        className="h-8"
                        alt="Zilliqa Logo"
                      />
                    )}
                    <p>{token.name}</p>
                    <FontAwesomeIcon
                      icon={faArrowUpRightFromSquare}
                      color="white"
                      className="ml-auto"
                    />
                  </button>
                </div>
                <input
                  className={`input join-item input-bordered w-full text-right ${
                    !hasEnoughBalance && isAmountNonZero
                      ? "input-error"
                      : !hasEnoughAllowance &&
                        isAmountNonZero &&
                        !!allowance &&
                        "input-warning"
                  }`}
                  placeholder="Amount"
                  type="number"
                  value={amount}
                  onChange={({ target }) => setAmount(target.value)}
                />
              </div>
              {!hasEnoughBalance && isAmountNonZero ? (
                <div className="label align-bottom place-content-end">
                  <span className="label-text-alt text-error">
                    Insufficient balance
                  </span>
                </div>
              ) : (
                !hasEnoughAllowance &&
                isAmountNonZero &&
                !!allowance && (
                  <div className="label align-bottom place-content-end">
                    <span className="label-text-alt text-warning">
                      Insufficient allowance
                    </span>
                  </div>
                )
              )}
            </div>
            <div className="card-actions mt-auto pt-4">
              {!hasEnoughAllowance && hasEnoughBalance ? (
                <button
                  className="btn w-5/6 mx-10 btn-outline"
                  disabled={showLoadingButton}
                  onClick={async () => {
                    if (approve) {
                      const tx = await approve();
                      console.log(tx.hash);
                      setLatestTxn(["approve", tx.hash]);
                    }
                  }}
                >
                  {showLoadingButton ? (
                    <>
                      <span className="loading loading-spinner"></span>
                      Loading
                    </>
                  ) : (
                    "Approve"
                  )}
                </button>
              ) : (
                <button
                  className="btn w-5/6 mx-10 btn-primary text-primary-content"
                  disabled={!canBridge || showLoadingButton}
                  onClick={async () => {
                    let tx: { hash: `0x${string}` };
                    if (fromChainConfig.isZilliqa && bridgeFromZilliqa) {
                      tx = await bridgeFromZilliqa();
                    } else if (bridge) {
                      tx = await bridge();
                    } else {
                      return;
                    }
                    console.log(tx.hash);
                    setLatestTxn(["bridge", tx.hash]);
                  }}
                >
                  {showLoadingButton ? (
                    <>
                      <span className="loading loading-spinner"></span>
                      Loading
                    </>
                  ) : (
                    "Bridge"
                  )}
                </button>
              )}
            </div>
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
