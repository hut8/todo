import React from 'react'
import TodoItem from './TodoItem'

const TodoList = (props) => {
    return (
        <>
            {props.tasks.map((task) => (
                <TodoItem task={task} />
            ))}
        </>
    )
}

export default TodoList
