import ReactDOM from "react-dom/client";
import { useEffect, useState } from "react";
import "./App.css";

import getAll from "./api/get";
import type { ApiResult } from "./api/utils";

import { CreateToDoItem } from "./components/CreateItemForm";
import { ToDoItem } from "./components/ToDoItem";
import { LoginForm } from "./components/LoginForm";

import init, {
  rust_generate_button_text,
} from "../rust-interface/pkg/rust_interface.js";

import type { ToDoItems } from "./interfaces/toDoItems";

const App = () => {
  const [data, setData] = useState<ToDoItems | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [wasmReady, setWasmReady] = useState(false);
  const [loggedin, setLoggedin] = useState<boolean>(
    localStorage.getItem("token") !== null,
  );

  function setToken(token: string) {
    localStorage.setItem("token", token);
    setLoggedin(true);
  }

  // Effect 1: initialize WASM once, independent of login state
  useEffect(() => {
    let cancelled = false;

    const initWasm = async () => {
      try {
        await init();
        if (cancelled) return;
        setWasmReady(true);
      } catch (e) {
        if (cancelled) return;
        console.error("Application initialization error:", e);
        setError("Failed to initialize application");
      }
    };

    initWasm();

    return () => {
      cancelled = true;
    };
  }, []);

  // Effect 2: fetch data only when WASM is ready AND the user is logged in
  useEffect(() => {
    if (!wasmReady || !loggedin) return;

    let cancelled = false;

    const fetchData = async () => {
      try {
        const response = await getAll();
        if (cancelled) return;

        if ("error" in response) {
          setError(response.error);
          return;
        }

        setData(response.data);
        setError(null);
      } catch (e) {
        if (cancelled) return;
        console.error("Fetch error: ", e);
        setError("Failed to fetch data");
      }
    };

    fetchData();

    return () => {
      cancelled = true;
    };
  }, [wasmReady, loggedin]);

  function reRenderItems(response: ApiResult<ToDoItems>) {
    if ("error" in response) {
      setError(response.error);
      return;
    }

    setData(response.data);
    setError(null);
  }

  if (!loggedin) {
    return <LoginForm setToken={setToken} />;
  }

  if (error) {
    return <div style={{ color: "red" }}>Error: {error}</div>;
  }

  if (!wasmReady || !data) {
    return <div>Loading...</div>;
  }

  return (
    <div className="App">
      <div className="mainContainer">
        <div className="header">
          <p>Complete tasks: {data.done.length}</p>
          <p>Pending tasks: {data.pending.length}</p>
        </div>

        <h1>Pending Items</h1>

        <div>
          {data.pending.map((item) => (
            <ToDoItem
              key={item.name}
              name={item.name}
              status={item.status}
              buttonMessage={rust_generate_button_text(item.status)}
              passBackResponse={reRenderItems}
            />
          ))}
        </div>

        <h1>Done Items</h1>

        <div>
          {data.done.map((item) => (
            <ToDoItem
              key={item.name}
              name={item.name}
              status={item.status}
              buttonMessage={rust_generate_button_text(item.status)}
              passBackResponse={reRenderItems}
            />
          ))}

          <CreateToDoItem passBackResponse={reRenderItems} />
        </div>
      </div>
    </div>
  );
};

const container = document.getElementById("root");

if (!container) {
  throw new Error("Missing #root element in index.html");
}

ReactDOM.createRoot(container).render(<App />);
