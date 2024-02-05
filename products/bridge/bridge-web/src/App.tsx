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
} from "wagmi";
import { formatUnits, parseUnits } from "viem";
import { MintAndBurnTokenManagerAbi } from "./abi/MintAndBurnTokenManager";
import { toast } from "react-toastify";

function App() {
  const [fromChain, setFromChain] = useState<Chains>(
    Object.values(chainConfigs)[0].chain
  );
  const { address: account } = useAccount();
  const [toChain, setToChain] = useState<Chains>(
    Object.values(chainConfigs)[1].chain
  );
  const [amount, setAmount] = useState<number>(0);
  const fromChainConfig = chainConfigs[fromChain]!;
  const toChainConfig = chainConfigs[toChain]!;
  const { switchNetwork } = useSwitchNetwork();
  const { chain } = useNetwork();

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
    !!decimals && !!amount
      ? (allowance ?? 0n) >= parseUnits(amount.toString(), decimals)
      : true;
  const hasEnoughBalance =
    decimals && balance
      ? parseUnits(amount.toString(), decimals) <= balance
      : false;
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
    abi: MintAndBurnTokenManagerAbi,
    args: [
      token.address,
      BigInt(toChainConfig.chainId),
      ethRecipient as `0x${string}`,
      parseUnits(amount.toString(), decimals ?? 0),
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
      abi: MintAndBurnTokenManagerAbi,
      args: [
        token.address,
        BigInt(toChainConfig.chainId),
        ethRecipient as `0x${string}`,
        parseUnits(amount.toString(), decimals || 0),
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
      parseUnits(amount.toString(), decimals ?? 0),
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
                    {fromChainConfig.name}
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
                    {toChainConfig.name}
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
                    !hasEnoughBalance && amount > 0
                      ? "input-error"
                      : !hasEnoughAllowance &&
                        amount > 0 &&
                        !!allowance &&
                        "input-warning"
                  }`}
                  placeholder="Amount"
                  type="number"
                  value={amount || ""}
                  onChange={({ target }) => setAmount(Number(target.value))}
                />
              </div>
              {!hasEnoughBalance && amount > 0 ? (
                <div className="label align-bottom place-content-end">
                  <span className="label-text-alt text-error">
                    Insufficient balance
                  </span>
                </div>
              ) : (
                !hasEnoughAllowance &&
                amount > 0 &&
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
                  disabled={isLoadingApprove}
                  onClick={async () => {
                    if (approve) {
                      const tx = await approve();
                      toast.success(
                        <div>
                          Approve txn sent. View on{" "}
                          <a
                            className="link text-ellipsis w-10"
                            onClick={() =>
                              window.open(
                                `${fromChainConfig.blockExplorer}${tx.hash}`,
                                "_blank"
                              )
                            }
                          >
                            block explorer
                          </a>
                        </div>
                      );
                      console.log(tx.hash);
                    }
                  }}
                >
                  {isLoadingApprove ? (
                    <>
                      <span className="loading loading-spinner"></span>
                      loading
                    </>
                  ) : (
                    "Approve"
                  )}
                </button>
              ) : (
                <button
                  className="btn w-5/6 mx-10 btn-primary text-primary-content"
                  disabled={!canBridge}
                  onClick={async () => {
                    let tx: { hash: string };
                    if (fromChainConfig.isZilliqa && bridgeFromZilliqa) {
                      tx = await bridgeFromZilliqa();
                      console.log(tx.hash);
                    } else if (bridge) {
                      tx = await bridge();
                      console.log(tx.hash);
                    } else {
                      return;
                    }
                    toast.success(
                      <div>
                        Bridge request txn sent. From {fromChainConfig.name} to{" "}
                        {toChainConfig.name} {amount} {token.name} tokens. View
                        on{" "}
                        <a
                          className="link text-ellipsis w-10"
                          onClick={() =>
                            window.open(
                              `${fromChainConfig.blockExplorer}${tx.hash}`,
                              "_blank"
                            )
                          }
                        >
                          block explorer
                        </a>
                      </div>
                    );
                    setAmount(0);
                  }}
                >
                  {isLoadingBridge || isLoadingBridgeFromZilliqa ? (
                    <>
                      <span className="loading loading-spinner"></span>
                      loading
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
