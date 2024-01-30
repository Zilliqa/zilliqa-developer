import { ConnectButton } from "@rainbow-me/rainbowkit";
import zilliqa from "./assets/zilliqa.png";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowRight, faChevronDown } from "@fortawesome/free-solid-svg-icons";

function App() {
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
                  BSC
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
                  <li>
                    <a>BSC</a>
                  </li>
                  <li>
                    <a>Zilliqa</a>
                  </li>
                </ul>
              </div>
              <FontAwesomeIcon icon={faArrowRight} color="white" />
              <div className="dropdown">
                <div tabIndex={0} role="button" className="btn m-1 w-52">
                  Zilliqa
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
                  <li>
                    <a>BSC</a>
                  </li>
                  <li>
                    <a>Zilliqa</a>
                  </li>
                </ul>
              </div>
            </div>

            <label>Recipient Address</label>
            <input
              type="text"
              placeholder="Zil Address"
              className="input w-full"
            />

            <label>Token</label>
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
              />
            </div>
            <div className="card-actions mt-auto pt-4">
              <button className="btn w-5/6 mx-10 btn-primary text-primary-content">
                BRIDGE
              </button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
