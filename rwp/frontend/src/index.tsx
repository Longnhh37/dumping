import ReactDOM from "react-dom/client";
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
      <TaskList title="Pending" items={data.pending} />
      <TaskList title="Done" items={data.done} />
    </div>
  );
};

const container = document.getElementById("root");
if (!container) throw new Error("Missing #root element in index.html");

ReactDOM.createRoot(container).render(<App />);
