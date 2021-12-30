import React from 'react'

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
