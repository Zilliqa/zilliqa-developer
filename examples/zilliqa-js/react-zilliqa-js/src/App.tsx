import React from "react";
import logo from "./logo.svg";
import "./App.css";
import { Zilliqa } from "@zilliqa-js/zilliqa";

type AppState = { blockHashes: string[] };
type AppProps = Record<string, never>;
class App extends React.Component<AppProps, AppState> {
  constructor(props: AppProps) {
    super(props);
    this.state = {
      blockHashes: [],
    };
  }

  async updateTxs(): Promise<void> {
    const count = 10;
    const provider = "https://api.zilliqa.com";
    const zilliqa = new Zilliqa(provider);
    const latestTxBlock = await zilliqa.blockchain.getNumTxBlocks();

    // Calculate the starting block to fetch transactions
    const startBlock = parseInt(latestTxBlock.result || "0") - count;

    // Fetch latest transactions
    const hashes: string[] = [];
    for (let i = 0; i < count; i++) {
      const blockNumber = startBlock + i;
      const txBlock = await zilliqa.blockchain.getTxBlock(blockNumber);

      if (txBlock.result) {
        hashes.push(txBlock.result.body.BlockHash);
        this.setState({ blockHashes: hashes });
      }
    }
    this.setState({ blockHashes: hashes });
  }

  componentDidMount(): void {
    this.updateTxs();
  }

  render(): JSX.Element {
    const { blockHashes } = this.state;
    return (
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <p>Found {blockHashes.length} hashes.</p>
          <ul>
            {blockHashes.map((e: string, index: number) => (
              <li key={index}>{e}</li>
            ))}
          </ul>
        </header>
      </div>
    );
  }
}

export default App;
