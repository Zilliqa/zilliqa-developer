import { useState } from "react";
import { ConnectButton } from "@rainbow-me/rainbowkit";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
      <h1 className="text-4xl font-bold underline">Hello world!</h1>
      <button className="btn">Button</button>
      <ConnectButton showBalance />
    </>
  );
}

export default App;
