import React from 'react'
import PropTypes from 'prop-types'

function TodoItem(props) {
    let task = props.task;
    return (
        <div class="todo-item">
            <div>{task.description}</div>
            <div>{task.created_at}</div>
            <div><input type="checkbox" checked={task.completed} /></div>
        </div>
    )
}

TodoItem.propTypes = {
    description: PropTypes.string.isRequired,
    created_at: PropTypes.string.isRequired,
    completed: PropTypes.bool.isRequired,
}

export default TodoItem

