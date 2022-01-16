import TodoList from "./TodoList";
import AddTodo from "./AddTodo";

function App() {
  return (
    <>
      <header className="row">
        <h1>Todo List</h1>
      </header>
      <div>
        <AddTodo />
        <TodoList />
      </div>
     </>
    );
  }
  
  export default App;
  