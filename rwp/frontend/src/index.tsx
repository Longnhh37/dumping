import ReactDOM from "react-dom/client";

const App = () => {
  return (
    <div>
      <h1>Hello World</h1>
    </div>
  );
};
const container = document.getElementById("root");
if (!container) throw new Error("Missing #root element in index.html");

ReactDOM.createRoot(container).render(<App />);
