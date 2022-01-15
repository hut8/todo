import TodoList from "./TodoList";
import AddTodo from "./AddTodo";

function App() {
  return (
    <div>
      <header>
        <h1>Todo List</h1>
      </header>
      <div>
        <AddTodo />
        <TodoList />
      </div>
    </div>
    );
  }
  
  export default App;
  