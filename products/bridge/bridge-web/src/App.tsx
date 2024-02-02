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
  usePrepareContractWrite,
  useSwitchNetwork,
} from "wagmi";
import { formatUnits, parseUnits } from "viem";
import { MintAndBurnTokenManagerAbi } from "./abi/MintAndBurnTokenManager";

function App() {
  const [fromChain, setFromChain] = useState<Chains>("zq-testnet");
  const account = useAccount();
  const [toChain, setToChain] = useState<Chains>("bsc-testnet");
  const [amount, setAmount] = useState<number>(0);
  const fromChainConfig = chainConfigs[fromChain];
  const toChainConfig = chainConfigs[toChain];
  const { switchNetwork } = useSwitchNetwork();

  const [recipient, setRecipient] = useState<string>();
  const [token, selectedToken] = useState<TokenConfig>(
    chainConfigs["zq-testnet"].tokens[0]
  );

  const blur = () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const activeElement: any | null = document.activeElement;
    if (activeElement) {
      activeElement.blur();
    }
  };

  useEffect(() => {
    switchNetwork && switchNetwork(fromChainConfig.chainId);
  }, [fromChainConfig, switchNetwork]);

  useEffect(() => {
    selectedToken(fromChainConfig.tokens[0]);
  }, [fromChain, fromChainConfig.tokens]);

  const { data: balance } = useContractRead({
    abi: erc20ABI,
    functionName: "balanceOf",
    args: account ? [account.address!] : undefined,
    address: token.address,
    enabled: !!account.address,
    watch: true,
  });

  useEffect(() => {
    setRecipient(account.address);
  }, [account.address]);

  const { data: decimals } = useContractRead({
    abi: erc20ABI,
    functionName: "decimals",
    address: token.address,
  });

  const { data: allowance } = useContractRead({
    abi: erc20ABI,
    functionName: "allowance",
    address: token.address,
    args: [account.address!, fromChainConfig.tokenManagerAddress],
    enabled: !!account.address,
    watch: true,
  });

  const hasEnoughAllowance =
    decimals && allowance
      ? allowance > parseUnits(amount.toString(), decimals)
      : false;
  const hasValidAddress = recipient
    ? validation.isBech32(recipient) || validation.isAddress(recipient)
    : false;

  const { config: transferConfig } = usePrepareContractWrite({
    address: fromChainConfig.tokenManagerAddress,
    abi: MintAndBurnTokenManagerAbi,
    args:
      toChainConfig && recipient && amount && decimals
        ? [
            token.address,
            BigInt(toChainConfig.chainId),
            recipient as `0x${string}`,
            parseUnits(amount.toString(), decimals),
          ]
        : undefined,
    functionName: "transfer",
  });

  const { writeAsync: bridgeZilliqa } = useContractWrite({
    mode: "prepared",
    request: {
      address: fromChainConfig.tokenManagerAddress,
      abi: MintAndBurnTokenManagerAbi,
      args:
        toChainConfig && recipient && amount && decimals
          ? [
              token.address,
              BigInt(toChainConfig.chainId),
              recipient as `0x${string}`,
              parseUnits(amount.toString(), decimals),
            ]
          : undefined,
      functionName: "transfer",
      gas: 600_000n,
      type: "legacy",
    },
  });

  const { writeAsync: bridge, isLoading: isLoadingBridge } =
    useContractWrite(transferConfig);

  const { config: approveConfig, isError: isErrorApprove } =
    usePrepareContractWrite({
      address: token.address,
      abi: erc20ABI,
      args: [
        fromChainConfig.tokenManagerAddress,
        parseUnits(amount.toString(), decimals || 0),
      ],
      functionName: "approve",
      type: "legacy",
    });

  const { writeAsync: approve, isLoading: isLoadingApprove } =
    useContractWrite(approveConfig);

  const canBridge =
    !hasValidAddress ||
    (fromChainConfig.isZilliqa ? false : !amount || isLoadingBridge);

  return (
    <>
      <div className="h-screen flex items-center justify-center">
        <div className="fixed top-0 navbar py-6 px-10 ">
          <div className="flex-1">
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
                <div className="dropdown">
                  <div tabIndex={0} role="button" className="btn w-52">
                    {fromChainConfig.name}
                    <FontAwesomeIcon
                      icon={faChevronDown}
                      className="ml-auto"
                      color="white"
                    />
                  </div>
                  <ul
                    tabIndex={0}
                    className="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52"
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
                            blur();
                          }}
                        >
                          <a>{name}</a>
                        </li>
                      ))}
                  </ul>
                </div>
                <FontAwesomeIcon
                  className="w-10 self-center"
                  icon={faArrowRight}
                  color="white"
                />
                <div className="dropdown">
                  <div tabIndex={0} role="button" className="btn w-52">
                    {toChainConfig.name}
                    <FontAwesomeIcon
                      icon={faChevronDown}
                      color="white"
                      className="ml-auto"
                    />
                  </div>
                  <ul
                    tabIndex={0}
                    className="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52"
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
                  {balance && decimals ? formatUnits(balance, decimals) : null}
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
                    !hasEnoughAllowance && "input-warning"
                  }`}
                  placeholder="Amount"
                  type="number"
                  onChange={({ target }) => setAmount(Number(target.value))}
                />
              </div>
              {!hasEnoughAllowance && (
                <div className="label align-bottom place-content-end">
                  <span className="label-text-alt text-warning">
                    Insufficient allowance
                  </span>
                </div>
              )}
            </div>
            <div className="card-actions mt-auto pt-4">
              {hasEnoughAllowance ? (
                <button
                  className="btn w-5/6 mx-10 btn-primary text-primary-content"
                  disabled={canBridge}
                  onClick={async () => {
                    if (fromChainConfig.isZilliqa && bridgeZilliqa) {
                      const tx = await bridgeZilliqa();
                      console.log(tx.hash);
                    } else if (bridge) {
                      const res = await bridge();
                      console.log(res.hash);
                    }
                  }}
                >
                  Bridge
                </button>
              ) : (
                <button
                  className="btn w-5/6 mx-10 btn-outline"
                  disabled={
                    !amount ||
                    !approve ||
                    isLoadingApprove ||
                    isErrorApprove ||
                    hasEnoughAllowance
                  }
                  onClick={async () => {
                    if (approve) {
                      const res = await approve();
                      console.log(res.hash);
                    }
                  }}
                >
                  Approve
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
