import ReactDOM from "react-dom/client";
import { useEffect, useState } from "react";
import "./App.css";
import getAll from "./api/get";
import type { ApiResult } from "./api/utils";
import { CreateToDoItem } from "./components/CreateItemForm";
import { ToDoItem } from "./components/ToDoItem";
import type { ToDoItems } from "./interfaces/toDoItems";

const App = () => {
  const [data, setData] = useState<ToDoItems | null>(null);
  const [error, setError] = useState<string | null>(null);

  function reRenderItems(response: ApiResult<ToDoItems>) {
    if ("error" in response) {
      alert(JSON.stringify(response));
      return;
    }
    setData(response.data);
    setError(null);
  }

  useEffect(() => {
    const fetchData = async () => {
      const response = await getAll();
      if ("error" in response) {
        setError(response.error);
      } else {
        setData(response.data);
      }
    };
    fetchData();
  }, []);

  if (error) return <div style={{ color: "red" }}>Error: {error}</div>;
  if (!data) return <div>Loading...</div>;

  return (
    <div className="App">
      <div className="mainContainer">
        <div className="header">
          <p>complete tasks: {data.done.length}</p>
          <p>pending tasks: {data.pending.length}</p>
        </div>

        <h1>Pending Items</h1>
        {data.pending.map((item) => (
          <ToDoItem
            key={item.title + item.status}
            title={item.title}
            status={item.status}
            passBackResponse={reRenderItems}
          />
        ))}

        <h1>Done Items</h1>
        {data.done.map((item) => (
          <ToDoItem
            key={item.title + item.status}
            title={item.title}
            status={item.status}
            passBackResponse={reRenderItems}
          />
        ))}

        <CreateToDoItem passBackResponse={reRenderItems} />
      </div>
    </div>
  );
};

const container = document.getElementById("root");
if (!container) throw new Error("Missing #root element in index.html");

ReactDOM.createRoot(container).render(<App />);
