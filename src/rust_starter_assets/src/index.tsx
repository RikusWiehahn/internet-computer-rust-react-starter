import "./main.css";
import * as React from "react";
import * as ReactDOM from "react-dom";
import { BrowserRouter, Link } from "react-router-dom";
import { rust_starter } from "../../declarations/rust_starter";

const App = () => {
  return (
    <BrowserRouter>
      <div className="bg-gray-100">
        Hello world!
      </div>
    </BrowserRouter>
  );
};

ReactDOM.render(<App />, document.getElementById("root"));
