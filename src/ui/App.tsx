import React from "react";
import "./App.scss";
import MainPanel from "./MainPanel/MainPanel";
import PrefixList from "./PrefixList/PrefixList";

function App() {
  return (
    <main>
      <PrefixList />
      <MainPanel />
    </main>
  );
}

export default App;
