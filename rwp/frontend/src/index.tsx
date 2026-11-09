import ReactDOM from "react-dom/client";
import "./App.css";
import { CreateToDoItem } from "./components/CreateItemForm";
import { useEffect, useState } from "react";
import getAll from "./api/get";
import type { ToDoItem, ToDoItems } from "./interfaces/toDoItems";

const TaskList = ({ title, items }: { title: string; items: ToDoItem[] }) => (
  <section>
    <h2>
      {title} ({items.length})
    </h2>
    <ul>
      {items.map((item) => (
        <li key={item.title}>{item.title}</li>
      ))}
    </ul>
  </section>
);

const App = () => {
  const [data, setData] = useState<ToDoItems | null>(null);
  const [error, setError] = useState<string | null>(null);

  function reRenderItems(response: any) {
    if (response.error) {
      alert(JSON.stringify(response));
      return;
    } else if (response.data) {
      setData(response.data);
      setError(null);
    } else {
      setError("Unknown error");
    }
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
    <div>
      {error ? (
        <div style={{ color: "red" }}>Error: {error}</div>
      ) : data ? (
        <div>Data loaded: {JSON.stringify(data)}</div>
      ) : (
        <div>Loading...</div>
      )}
      <CreateToDoItem passBackResponse={reRenderItems} />
    </div>
  );
};

const container = document.getElementById("root");
if (!container) throw new Error("Missing #root element in index.html");

ReactDOM.createRoot(container).render(<App />);
