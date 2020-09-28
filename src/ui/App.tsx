import React, { useEffect, useState } from "react";
import Prefix from "../core/prefix/prefix";
import "./App.scss";
import MainPanel from "./MainPanel/MainPanel";
import SidePanel from "./SidePanel/SidePanel";
import ErrorModal, { Error } from "./ErrorModal";

function App() {
  const [prefixes, setPrefixes] = useState<Prefix[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | undefined>();

  useEffect(() => {
    // TODO: get path from user
    Prefix.allFromDir("~/.wine")
      .then((pfxs) => {
        setPrefixes(pfxs);
        setError(undefined);
      })
      .catch((err) =>
        setError({
          context: "Error Loading Prefixes",
          message: err.message,
        })
      )
      .finally(() => setLoading(false));
  }, []);

  return (
    <main>
      <SidePanel prefixes={prefixes} />
      {loading && <span>Loading</span>}
      <MainPanel />
      {error && <ErrorModal {...error} />}
    </main>
  );
}

export default App;
