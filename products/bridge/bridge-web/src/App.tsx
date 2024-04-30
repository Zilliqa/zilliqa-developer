import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faArrowRight,
  faArrowUpRightFromSquare,
  faChevronDown,
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
  usePublicClient,
  useSwitchNetwork,
  useWaitForTransaction,
} from "wagmi";
import { formatEther, formatUnits, getAbiItem, parseUnits } from "viem";
import { Id, toast } from "react-toastify";
import { tokenManagerAbi } from "./abi/TokenManager";
import Navbar from "./components/Navbar";
import useRecipientInput from "./hooks/useRecipientInput";
import RecipientInput from "./components/RecipientInput";
import { chainGatewayAbi } from "./abi/ChainGateway";

type TxnType = "approve" | "bridge";

function App() {
  const { address: account } = useAccount();
  const { switchNetwork } = useSwitchNetwork();
  const { chain } = useNetwork();

  const [fromChain, setFromChain] = useState<Chains>(
    Object.values(chainConfigs)[0].chain
  );
  const [toChain, setToChain] = useState<Chains>(
    Object.values(chainConfigs)[1].chain
  );
  const [amount, setAmount] = useState<string | undefined>();
  const isAmountNonZero = Number(amount) > 0;
  const [latestTxn, setLatestTxn] = useState<[TxnType, `0x${string}`]>();
  const [loadingId, setLoadingId] = useState<Id>();
  const [token, selectedToken] = useState<TokenConfig>(
    Object.values(chainConfigs)[0].tokens[0]
  );

  const { recipientEth, isAddressValid } = useRecipientInput();

  const fromChainConfig = chainConfigs[fromChain]!;
  const toChainConfig = chainConfigs[toChain]!;

  const fromChainClient = usePublicClient();
  const toChainClient = usePublicClient({ chainId: toChainConfig.chainId });

  useEffect(() => {
    switchNetwork && switchNetwork(fromChainConfig.chainId);
  }, [fromChainConfig, switchNetwork]);

  useEffect(() => {
    selectedToken(fromChainConfig.tokens[0]);
  }, [fromChain, fromChainConfig.tokens]);

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
  const { data: fees } = useContractRead({
    abi: tokenManagerAbi,
    functionName: "getFees",
    address: fromChainConfig.tokenManagerAddress,
    enabled: !!fromChainConfig.tokenManagerAddress,
  });
  const { data: paused } = useContractRead({
    abi: tokenManagerAbi,
    functionName: "paused",
    address: fromChainConfig.tokenManagerAddress,
    enabled: !!fromChainConfig.tokenManagerAddress,
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
    decimals && isAmountNonZero
      ? (allowance ?? 0n) >= parseUnits(amount!, decimals)
      : true;
  const hasEnoughBalance =
    decimals && balance && amount
      ? parseUnits(amount, decimals) <= balance
      : false;

  const { config: transferConfig } = usePrepareContractWrite({
    address: fromChainConfig.tokenManagerAddress,
    abi: tokenManagerAbi,
    args: recipientEth && [
      token.address,
      BigInt(toChainConfig.chainId),
      recipientEth,
      amount ? parseUnits(amount, decimals ?? 0) : 0n,
    ],
    functionName: "transfer",
    value: fees ?? 0n,
    enabled: !!(
      hasEnoughAllowance &&
      toChainConfig &&
      fromChainConfig &&
      !fromChainConfig.isZilliqa &&
      recipientEth &&
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
      abi: tokenManagerAbi,
      value: fees ?? 0n,
      args: [
        token.address,
        BigInt(toChainConfig.chainId),
        recipientEth!,
        amount ? parseUnits(amount, decimals ?? 0) : 0n,
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
      amount ? parseUnits(amount, decimals ?? 0) : 0n,
    ],
    functionName: "approve",
    gas: fromChainConfig.isZilliqa ? 400_000n : undefined,
    type: fromChainConfig.isZilliqa ? "legacy" : "eip1559",
    enabled: !hasEnoughAllowance,
  });

  const { writeAsync: approve, isLoading: isLoadingApprove } =
    useContractWrite(approveConfig);

  const canBridge =
    isAmountNonZero &&
    isAddressValid &&
    hasEnoughAllowance &&
    hasEnoughBalance &&
    !paused &&
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
        (async () => {
          const logs = await fromChainClient.getLogs({
            address: fromChainConfig.chainGatewayAddress,
            event: getAbiItem({
              abi: chainGatewayAbi,
              name: "Relayed",
              args: [toChainConfig.chainId],
            }),
            blockHash: txnReceipt.blockHash,
          });
          const nonce = logs.find(
            (log) => log.transactionHash === txnReceipt.transactionHash
          )?.args.nonce;

          const id = toast.loading(`Bridging to ${toChainConfig.name}...`);

          // TODO: find a way to stop watching once event arrives
          toChainClient.watchContractEvent({
            abi: chainGatewayAbi,
            address: toChainConfig.chainGatewayAddress,
            eventName: "Dispatched",
            args: {
              nonce,
            },
            onLogs: (logs) => {
              toast.update(id, {
                render: (
                  <div>
                    Bridge txn complete, funds arrived to {toChainConfig.name}{" "}
                    chain. View on{" "}
                    <a
                      className="link text-ellipsis w-10"
                      onClick={() =>
                        window.open(
                          `${toChainConfig.blockExplorer}${logs[0].transactionHash}`,
                          "_blank"
                        )
                      }
                    >
                      block explorer
                    </a>
                  </div>
                ),
                type: "success",
                isLoading: false,
              });
            },
          });

          // Double check if it has already been dispatched before event listener catches it
          const blockNumber = await toChainClient.getBlockNumber();
          const dispatched = await toChainClient.getLogs({
            address: toChainConfig.chainGatewayAddress,
            event: getAbiItem({
              abi: chainGatewayAbi,
              name: "Dispatched",
            }),
            args: {
              nonce,
            },
            fromBlock: blockNumber - 50n,
            toBlock: "latest",
          });

          if (dispatched.length > 0) {
            toast.update(id, {
              render: (
                <div>
                  Bridge txn complete, funds arrived to {toChainConfig.name}{" "}
                  chain. View on{" "}
                  <a
                    className="link text-ellipsis w-10"
                    onClick={() =>
                      window.open(
                        `${toChainConfig.blockExplorer}${dispatched[0].transactionHash}`,
                        "_blank"
                      )
                    }
                  >
                    block explorer
                  </a>
                </div>
              ),
              type: "success",
              isLoading: false,
            });
          }
        })();

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
    fromChainConfig.chainGatewayAddress,
    toChainConfig.chainId,
    toChainConfig.chainGatewayAddress,
    fromChainClient,
    toChainClient,
    toChainConfig.blockExplorer,
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

  const selectTokenOnDropdown = (token: TokenConfig) => {
    const elem = document.activeElement;

    if (elem) {
      elem && (elem as any).blur();
    }

    selectedToken(token);
  };

  return (
    <>
      <div className="h-screen flex items-center justify-center">
        <Navbar />

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

            <RecipientInput />

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
                  <div className="join-item">
                    <div className="dropdown ">
                      <button tabIndex={0} role="button" className="btn w-40">
                        {token.logo && (
                          <img
                            src={token.logo}
                            className="h-8"
                            alt="Zilliqa Logo"
                          />
                        )}
                        <p>{token.name}</p>

                        <FontAwesomeIcon
                          icon={faChevronDown}
                          color="white"
                          className="ml-auto"
                        />
                      </button>

                      <ul
                        tabIndex={0}
                        className="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52"
                      >
                        {fromChainConfig.tokens.map((token) => (
                          <li
                            key={token.address}
                            onClick={() => selectTokenOnDropdown(token)}
                          >
                            <div className="flex items-center gap-2">
                              {token.logo && (
                                <img
                                  src={token.logo}
                                  className="h-8"
                                  alt="Zilliqa Logo"
                                />
                              )}
                              <p>{token.name}</p>
                            </div>
                          </li>
                        ))}
                      </ul>
                    </div>
                  </div>

                  <button
                    onClick={() => window.open(token.blockExplorer, "_blank")}
                    className="btn join-item"
                  >
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

            {!!fees && !!amount && (
              <>
                <div className="divider"></div>
                <div className="flex flex-col gap-1">
                  <div className="flex justify-between">
                    <label className="label-text-alt">Fees:</label>
                    <label className="label-text-alt">
                      {formatEther(fees).toString()}{" "}
                      {fromChainConfig.nativeTokenSymbol}
                    </label>
                  </div>

                  <div className="flex justify-between">
                    <label className="label-text-alt">
                      Recipient Receives:
                    </label>
                    <label className="label-text-alt">
                      {amount} {token.name}
                    </label>
                  </div>

                  <div className="flex justify-between">
                    <label className="label-text-alt">Total:</label>
                    <label className="label-text-alt">
                      {amount} {token.name} + {formatEther(fees).toString()}{" "}
                      {fromChainConfig.nativeTokenSymbol}
                    </label>
                  </div>
                </div>
              </>
            )}
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
            {paused && (
              <div role="alert" className="alert alert-warning mt-3">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="stroke-current shrink-0 h-6 w-6"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                  />
                </svg>
                <span>Warning: bridge is currently under maintenance.</span>
              </div>
            )}
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
