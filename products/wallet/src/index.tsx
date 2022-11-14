import React from "react";
import ReactDOM from "react-dom/client";

import "bootstrap/dist/css/bootstrap.min.css";
import "rc-steps/assets/index.css";
import "./index.css";

import { RouterNode } from "./routes";

import reportWebVitals from "./reportWebVitals";

const root_element = document.getElementById("root");
if (root_element) {
  const root = ReactDOM.createRoot(root_element);
  root.render(
    <React.StrictMode>
      <RouterNode />
    </React.StrictMode>
  );
}

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
