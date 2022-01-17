import "./main.css";
import * as React from "react";
import * as ReactDOM from "react-dom";
import { rust_profile } from "../../declarations/rust_profile";

const App = () => {
  return (
    <div className="bg-gray-400">
      <h1 className="text-3xl font-bold underline">Hello world!</h1>
    </div>
  );
};

ReactDOM.render(<App />, document.getElementById("root"));
