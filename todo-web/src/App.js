import TodoList from "./TodoList";
import AddTodo from "./AddTodo";

function App() {
  return (
    <div>
      <header>
        <h1>Todo List</h1>
      </header>
      <TodoList />
      <AddTodo />
    </div>
  );
}

export default App;
