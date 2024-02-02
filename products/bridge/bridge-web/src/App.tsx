import { ConnectButton } from "@rainbow-me/rainbowkit";
import zilliqa from "./assets/zilliqa.png";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowRight, faChevronDown } from "@fortawesome/free-solid-svg-icons";
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
  });

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
              <p className="text-4xl">Zilliqa Bridge</p>
            </div>

            <label>Network</label>
            <div className="flex justify-between items-center gap-3">
              <div className="dropdown">
                <div tabIndex={0} role="button" className="btn m-1 w-52">
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
                    .filter((config) => config.chain !== fromChainConfig.chain)
                    .map(({ chain, name }) => (
                      <li
                        key={`from${chain}`}
                        onClick={() => {
                          if (chain === toChain) {
                            setToChain(
                              Object.values(chainConfigs).filter(
                                (chains) => chains.chain !== chain
                              )[0].chain
                            );
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
              <FontAwesomeIcon icon={faArrowRight} color="white" />
              <div className="dropdown">
                <div tabIndex={0} role="button" className="btn m-1 w-52">
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
                            setFromChain(
                              Object.values(chainConfigs).filter(
                                (chains) => chains.chain !== chain
                              )[0].chain
                            );
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

            <label>Recipient Address</label>
            <input
              type="text"
              placeholder="Zil Address"
              className="input w-full"
              value={recipient}
              onChange={({ target }) => setRecipient(target.value)}
            />

            <label>Token</label>
            <div className="flex flex-col">
              <div className="join">
                <div className="indicator">
                  <button className="btn join-item w-32">
                    FPS
                    <FontAwesomeIcon
                      icon={faChevronDown}
                      color="white"
                      className="ml-auto"
                    />
                  </button>
                </div>
                <input
                  className="input join-item input-bordered w-full text-right"
                  placeholder="Amount"
                  type="number"
                  onChange={({ target }) => setAmount(Number(target.value))}
                />
              </div>
              <div className="label align-bottom place-content-end">
                <span className="label-text-alt">
                  Balance:{" "}
                  {balance && decimals ? formatUnits(balance, decimals) : null}
                </span>
              </div>
            </div>
            <div className="card-actions mt-auto pt-4">
              <button
                className="btn w-5/6 mx-10 btn-outline text-primary-content"
                disabled={
                  !amount || !approve || isLoadingApprove || isErrorApprove
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
              <button
                className="btn w-5/6 mx-10 btn-primary text-primary-content"
                disabled={
                  fromChainConfig.isZilliqa ? false : !amount || isLoadingBridge
                }
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
            </div>
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
