import React from "react";
import ReactDOM from "react-dom";

import "bootstrap/dist/css/bootstrap.min.css";
import "rc-steps/assets/index.css";
import "./index.css";

import { RouterNode } from "./routes";

import reportWebVitals from "./reportWebVitals";
import { Buffer } from "buffer";

// @ts-ignore
(window as any).global = window;
// @ts-ignore
(window as any).global.Buffer = Buffer;
console.log("Hello world!!");
ReactDOM.render(
  <React.StrictMode>
    <RouterNode />
  </React.StrictMode>,
  document.getElementById("root")
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
